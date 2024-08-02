use std::fmt::{Debug, Display, Formatter};
use candle_core::{Device, DType, Tensor};
use candle_transformers::models::stable_diffusion;
use candle_transformers::models::stable_diffusion::clip::ClipTextTransformer;
use crate::cake::{Context, Forwarder};
use crate::models::llama3::{Cache};
use crate::models::sd::sd::ModelFile;
use crate::models::sd::util::{get_device, get_sd_config};
use crate::StableDiffusionVersion;

pub struct Clip {
    clip_model: ClipTextTransformer,
    layer_name: &'static str
}

impl Debug for Clip {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for Clip {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Forwarder for Clip {
    fn load(name: String, ctx: &Context) -> anyhow::Result<Box<Self>>
    where
        Self: Sized
    {
        let model_file;
        let model_filename;
        let sd_config = get_sd_config(ctx)?;
        let clip_config;

        match name {
            String::from("clip") => {
                model_file = ModelFile::Clip;
                model_filename = ctx.args.sd_args.clip.clone();
                clip_config = sd_config.clip;
            },
            String::from("clip2") => {
                model_file = ModelFile::Clip2;
                model_filename = ctx.args.sd_args.clip2.clone();
                clip_config = sd_config.clip2.unwrap();
            }
        };

        let dtype = if ctx.args.sd_args.use_f16 { DType::F16 } else { DType::F32 };
        let device = get_device(ctx.args.cpu)?;

        Self::load_model(
            model_file,
            model_filename,
            ctx.args.sd_args.sd_version,
            ctx.args.sd_args.use_f16,
            &device,
            dtype,
            &clip_config
        )
    }

    async fn forward(&self, x: &Tensor, index_pos: usize, block_idx: usize, cache: &mut Cache) -> anyhow::Result<Tensor> {
        todo!()
    }

    async fn forward_mut(&mut self, x: &Tensor, index_pos: usize, block_idx: usize, cache: &mut Cache) -> anyhow::Result<Tensor> {
        todo!()
    }

    fn layer_name(&self) -> &str {
        self.layer_name
    }
}

impl Clip {
    pub fn load_model(
        model_file: ModelFile,
        name: Option<String>,
        version: StableDiffusionVersion,
        use_f16: bool,
        device: &Device,
        dtype: DType,
        config: &stable_diffusion::clip::Config) -> anyhow::Result<Box<Self>>
    where
        Self: Sized
    {
        let clip_weights = model_file.get(name, version, use_f16)?;
        let clip_model = stable_diffusion::build_clip_transformer(config, clip_weights, device, dtype)?;
        let layer_name = model_file.name();
        Ok(Box::new(Self {
            clip_model,
            layer_name,
        }))
    }
}
