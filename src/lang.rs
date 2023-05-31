#[derive(Default, Debug)]
pub struct ActLang {
    language: String,
}

impl ActLang {
    pub fn lang(&self) -> &str {
        &self.language
    }

    pub fn lang_mut(&mut self) -> &mut String {
        &mut self.language
    }
}
pub enum Locale {
    English,
    Portuguese
}

impl Locale {
    pub fn from_string(string: &str) -> Self {
        match &string[0..2] {
            "en" => Self::English,
            "pt" => Self::Portuguese,
            _ => Self::English
        }
    }

    pub fn to_string(&self) -> &'static str {
        match *self {
            Self::English => "en",
            Self::Portuguese => "pt"
        }
    }
}

pub enum Messages {
    MainMenuOpt1,
    MainMenuOpt2,
    MainMenuOpt3,
    SettingsOpt1,
    SettingsOpt2
}

impl Messages {
    pub fn translate(&self, locale: &Locale) -> String {
        match *self {
            Self::MainMenuOpt1 => {
                match *locale {
                    Locale::English => 
                        "NEW GAME",
                     Locale::Portuguese => 
                        "NOVO JOGO"
                }.to_string()
            }
            Self::MainMenuOpt2 => {
                match *locale {
                    Locale::English => "SETTINGS",
                    Locale::Portuguese => "OPÇÕES"
                }.to_string()
            }
            Self::MainMenuOpt3 => {
                match *locale {
                    Locale::English => "EXIT",
                    Locale::Portuguese => "SAIR"
                }.to_string()
            }
            Self::SettingsOpt1 => {
                match *locale {
                    Locale::English => "LANGUAGE",
                    Locale::Portuguese => "IDIOMA"
                }.to_string()
            }
            Self::SettingsOpt2 => {
                match *locale {
                    Locale::English => "BACK",
                    Locale::Portuguese => "VOLTAR"
                }.to_string()
            }
        }
    }
}