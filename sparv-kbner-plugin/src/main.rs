use rust_bert::{
    pipelines::{
        common::ModelType, ner::NERModel, token_classification::TokenClassificationConfig,
    },
    resources::{LocalResource, RemoteResource},
};

pub type AppResult<T> = Result<T, Box<dyn std::error::Error>>;
fn main() -> AppResult<()> {
    let ner_model = NERModel::new(Default::default())?;
    let input = [
        "My name is Amy. I live in Paris.",
        "Paris is a city in France.",
    ];
    let output = ner_model.predict(&input);
    println!("{:#?}", output);

    run_nlp_on_sentence(&input)?;
    Ok(())
}

pub fn run_nlp_on_sentence(input: &[&str]) -> AppResult<()> {
    let ner_config = TokenClassificationConfig {
        model_type: ModelType::Bert,
        model_resource: Box::new(LocalResource {
            local_path: "/home/kristoffer/projekt/sb/sparv-rs/data/rust_model.ot".into(),
        }),
        config_resource: Box::new(LocalResource {
            local_path: "/home/kristoffer/projekt/sb/sparv-rs/data/config.json".into(),
        }),
        vocab_resource: Box::new(LocalResource {
            local_path: "/home/kristoffer/projekt/sb/sparv-rs/data/vocab.txt".into(),
        }),
        lower_case: false,
        ..Default::default()
    };
    let ner_model = NERModel::new(ner_config)?;

    let output = ner_model.predict(input);
    println!("{:#?}", output);
    Ok(())
}
