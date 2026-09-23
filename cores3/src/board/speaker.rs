use crate::{
    delay::delay_ms,
    drivers::aw88298,
    error::{Error, ErrorKind, Result},
};
use embedded_hal::i2c::I2c;
use esp_hal::{
    dma_circular_buffers,
    i2s::master::{Channels, Config, DataFormat, I2s, I2sTx},
    peripherals::{DMA_CH1, GPIO13, GPIO33, GPIO34, I2S0},
    time::Rate,
};

pub const SAMPLE_RATE: u32 = 44100;

const DMA_BUF_SIZE: usize = 4092 * 2;
const TONE_AMPLITUDE: i16 = i16::MAX / 4;
const AMP_SETTLE_MS: u32 = 10;

pub struct Speaker {
    tx: I2sTx<'static, esp_hal::Blocking>,
    buf: &'static mut [u8; DMA_BUF_SIZE],
}

impl Speaker {
    pub fn new(
        i2s: I2S0<'static>,
        dma_channel: DMA_CH1<'static>,
        bclk: GPIO34<'static>,
        ws: GPIO33<'static>,
        dout: GPIO13<'static>,
    ) -> Result<Self> {
        let config = Config::new_tdm_philips()
            .with_sample_rate(Rate::from_hz(SAMPLE_RATE))
            .with_data_format(DataFormat::Data16Channel16)
            .with_channels(Channels::MONO); // one sample per frame, duplicated to L/R
        let i2s = I2s::new(i2s, dma_channel, config).map_err(Error::hal)?;

        let (_, _, buf, descriptors) = dma_circular_buffers!(0, DMA_BUF_SIZE);
        let tx = i2s
            .i2s_tx
            .with_bclk(bclk)
            .with_ws(ws)
            .with_dout(dout)
            .build(descriptors);

        Ok(Self { tx, buf })
    }

    pub fn play(&mut self, i2c: &mut impl I2c, samples: &[i16]) -> Result<()> {
        self.play_iter(i2c, samples.iter().copied())
    }

    pub fn play_iter(
        &mut self,
        i2c: &mut impl I2c,
        samples: impl Iterator<Item = i16>,
    ) -> Result<()> {
        let mut samples = samples.fuse();

        // start clocking out silence so the amp is only enabled while BCLK is stable
        self.buf.fill(0);
        let mut transfer = self.tx.write_dma_circular(&self.buf).map_err(Error::hal)?;

        let result = aw88298::set_enabled(i2c, true).and_then(|_| {
            delay_ms(AMP_SETTLE_MS);

            let mut done = false;
            while !done {
                transfer
                    .push_with(|dst| {
                        let n = fill(dst, &mut samples);
                        done = n < dst.len() & !1;
                        n
                    })
                    .map_err(Error::hal)?;
            }

            // a full buffer of silence guarantees every sample has been played
            let mut silence = DMA_BUF_SIZE;
            while silence > 0 {
                silence -= transfer
                    .push_with(|dst| {
                        let n = dst.len().min(silence);
                        dst[..n].fill(0);
                        n
                    })
                    .map_err(Error::hal)?;
            }

            Ok(())
        });

        // even on error, the amp must be powered down before BCLK stops
        let disabled = aw88298::set_enabled(i2c, false);
        delay_ms(AMP_SETTLE_MS);
        let stopped = transfer.stop().map_err(Error::hal);

        result.and(disabled).and(stopped)
    }

    pub fn tone(&mut self, i2c: &mut impl I2c, freq_hz: u32, duration_ms: u32) -> Result<()> {
        if freq_hz == 0 || freq_hz > SAMPLE_RATE / 2 {
            return Err(Error::from(ErrorKind::InvalidArgument).with_context("tone frequency"));
        }

        let total = SAMPLE_RATE as u64 * duration_ms as u64 / 1000;
        let wave = (0..total).map(|i| {
            if (i * freq_hz as u64 * 2 / SAMPLE_RATE as u64).is_multiple_of(2) {
                TONE_AMPLITUDE
            } else {
                -TONE_AMPLITUDE
            }
        });

        self.play_iter(i2c, wave)
    }
}

fn fill(dst: &mut [u8], samples: &mut impl Iterator<Item = i16>) -> usize {
    let mut n = 0;
    for chunk in dst.chunks_exact_mut(2) {
        let Some(sample) = samples.next() else {
            break;
        };
        chunk.copy_from_slice(&sample.to_le_bytes());
        n += 2;
    }
    n
}
