use std::{collections::HashSet, fs::OpenOptions};
use std::io::Write;
use std::fmt::Display;

use polars::prelude::*;
use teloxide::{
    dispatching::dialogue::InMemStorage,
    prelude::*,
    types::{ButtonRequest, KeyboardButton, KeyboardMarkup},
};
type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, Clone)]
enum AgeRange {
    EighteenToTwentyFive,
    TwentyfiveToFourty,
    FourtyToFifty,
    FiftyPlus,
}

impl Display for AgeRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Clone)]
enum InvestmentStatus {
    None,
    Minus,
    Zero,
    Plus,
    BigPlus,
}

impl Display for InvestmentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Clone)]
enum InvestmentInstrument {
    None,
    Stocks,
    RealEstate,
    CryptoCurrency,
    BankDeposits,
}

impl Display for InvestmentInstrument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Clone)]
enum FundingStatus {
    LessThanMillion,
    MillionToFiveMillion,
    FiveMillionToTenMillion,
    TenMillionPlus,
}


impl Display for FundingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Clone, Default)]
enum State {
    #[default]
    Start,
    ReceiveAge,
    ReceiveInvestmentStatus {
        age: AgeRange,
    },
    ReceiveInvestmentInstrument {
        age: AgeRange,
        investment_status: InvestmentStatus,
    },
    RecieveFundingStatus {
        age: AgeRange,
        investment_status: InvestmentStatus,
        instrument: InvestmentInstrument,
    },
    ReceiveContact {
        age: AgeRange,
        investment_status: InvestmentStatus,
        instrument: InvestmentInstrument,
        funding_status: FundingStatus,
    },
    End {
        age: AgeRange,
        investment_status: InvestmentStatus,
        instrument: InvestmentInstrument,
        funding_status: FundingStatus,
        contact: String,
    },
}

const TOKEN: &str = "";

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let bot = Bot::new(TOKEN.to_string());

    Dispatcher::builder(
        bot.clone(),
        Update::filter_message()
            .enter_dialogue::<Message, InMemStorage<State>, State>()
            .branch(dptree::case![State::Start].endpoint(start))
            .branch(dptree::case![State::ReceiveAge].endpoint(recieve_age))
            .branch(
                dptree::case![State::ReceiveInvestmentStatus { age }]
                    .endpoint(recieve_investment_status),
            )
            .branch(
                dptree::case![State::ReceiveInvestmentInstrument {
                    age,
                    investment_status
                }]
                .endpoint(recieve_investment_instrument),
            )
            .branch(
                dptree::case![State::RecieveFundingStatus {
                    age,
                    investment_status,
                    instrument
                }]
                .endpoint(recieve_funding_status),
            )
            .branch(
                dptree::case![State::ReceiveContact {
                    age,
                    investment_status,
                    instrument,
                    funding_status,
                }]
                .endpoint(receive_contact),
            ),
    )
    .dependencies(dptree::deps![InMemStorage::<State>::new()])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}

async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let AGES_SET: HashSet<&str> = HashSet::from_iter(vec!["18-25", "25-40", "40-50", "50+"]);
    let keyboard = KeyboardMarkup::new(vec![AGES_SET
        .clone()
        .iter()
        .map(|age| KeyboardButton::new(age.to_string()))])
    .one_time_keyboard(true);

    bot.send_message(msg.chat.id, "Добрый день!\nСколько вам лет?")
        .reply_markup(keyboard)
        .await?;
    dialogue.update(State::ReceiveAge).await?;
    Ok(())
}

async fn recieve_age(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let AGES_SET: HashSet<&str> = HashSet::from_iter(vec!["18-25", "25-40", "40-50", "50+"]);
    let INVESTMENT_STATUS_SET: HashSet<&str> = HashSet::from_iter(vec![
        "Не было опыта",
        "Минус",
        "В нуле",
        "Плюс",
        "Большой плюс",
    ]);
    match msg.text() {
        None => {
            bot.send_message(msg.chat.id, "Выберите возраст!").await?;
        }
        Some(text) => {
            if AGES_SET.contains(&text) {
                if text == "18-25" {
                    dialogue
                        .update(State::ReceiveInvestmentStatus {
                            age: AgeRange::EighteenToTwentyFive,
                        })
                        .await?;
                } else if text == "25-40" {
                    dialogue
                        .update(State::ReceiveInvestmentStatus {
                            age: AgeRange::TwentyfiveToFourty,
                        })
                        .await?;
                } else if text == "40-50" {
                    dialogue
                        .update(State::ReceiveInvestmentStatus {
                            age: AgeRange::FourtyToFifty,
                        })
                        .await?;
                } else if text == "50+" {
                    dialogue
                        .update(State::ReceiveInvestmentStatus {
                            age: AgeRange::FiftyPlus,
                        })
                        .await?;
                }
                bot.send_message(
                    msg.chat.id,
                    "Отлично, если вы занимались инвестициями, каких результатов вы добились на 2023 год?",
                ).reply_markup(KeyboardMarkup::new(vec![INVESTMENT_STATUS_SET.iter().map(|status| KeyboardButton::new(status.to_string()))]).one_time_keyboard(true)).await?;
            } else {
                bot.send_message(msg.chat.id, "Выберите возраст!").await?;
            }
        }
    }

    Ok(())
}

async fn recieve_investment_status(
    bot: Bot,
    dialogue: MyDialogue,
    age: AgeRange,
    msg: Message,
) -> HandlerResult {
    let INVESTMENT_STATUS_SET: HashSet<&str> = HashSet::from_iter(vec![
        "Не было опыта",
        "Минус",
        "В нуле",
        "Плюс",
        "Большой плюс",
    ]);
    let INVESTMENT_INSTRUMENT_SET: HashSet<&str> =
        HashSet::from_iter(vec!["Акции", "Недвижимость", "Криптовалюта", "Вклады"]);
    match msg.text() {
        None => {
            bot.send_message(msg.chat.id, "Выберите вариант!").await?;
        }
        Some(status) => {
            if INVESTMENT_STATUS_SET.contains(&status) {
                if status == "Не было опыта" {
                    dialogue
                        .update(State::ReceiveInvestmentInstrument {
                            age,
                            investment_status: InvestmentStatus::None,
                        })
                        .await?;
                } else if status == "Минус" {
                    dialogue
                        .update(State::ReceiveInvestmentInstrument {
                            age,
                            investment_status: InvestmentStatus::Minus,
                        })
                        .await?;
                } else if status == "В нуле" {
                    dialogue
                        .update(State::ReceiveInvestmentInstrument {
                            age,
                            investment_status: InvestmentStatus::Zero,
                        })
                        .await?;
                } else if status == "Плюс" {
                    dialogue
                        .update(State::ReceiveInvestmentInstrument {
                            age,
                            investment_status: InvestmentStatus::Plus,
                        })
                        .await?;
                } else if status == "Большой плюс" {
                    dialogue
                        .update(State::ReceiveInvestmentInstrument {
                            age,
                            investment_status: InvestmentStatus::BigPlus,
                        })
                        .await?;
                }

                bot.send_message(msg.chat.id, "Какой инструмент вы использовали?\nЕсли не было опыта - выберите наиболее привлекательный.")
                .reply_markup(KeyboardMarkup::new(vec![INVESTMENT_INSTRUMENT_SET.iter().map(|_x| KeyboardButton::new(status.to_string()))]).one_time_keyboard(true)).await?;
            } else {
                bot.send_message(msg.chat.id, "Выберите вариант!").await?;
            }
        }
    }
    Ok(())
}

async fn recieve_investment_instrument(
    bot: Bot,
    dialogue: MyDialogue,
    (age, investment_status): (AgeRange, InvestmentStatus),
    msg: Message,
) -> HandlerResult {
    let INVESTMENT_INSTRUMENT_SET: HashSet<&str> =
        HashSet::from_iter(vec!["Акции", "Недвижимость", "Криптовалюта", "Вклады"]);
    let FUNDING_STATUS_SET: HashSet<&str> = HashSet::from_iter(vec![
        "<1 миллиона",
        "1-5 миллионов",
        "5-10 миллионов",
        "Более 10 миллионов",
    ]);
    match msg.text() {
        None => {
            bot.send_message(msg.chat.id, "Выберите вариант!").await?;
        }
        Some(instrument) => {
            if INVESTMENT_INSTRUMENT_SET.contains(&instrument) {
                if instrument == "Акции" {
                    dialogue
                        .update(State::RecieveFundingStatus {
                            age,
                            investment_status,
                            instrument: InvestmentInstrument::Stocks,
                        })
                        .await?;
                } else if instrument == "Недвижимость" {
                    dialogue
                        .update(State::RecieveFundingStatus {
                            age,
                            investment_status,
                            instrument: InvestmentInstrument::RealEstate,
                        })
                        .await?;
                } else if instrument == "Криптовалюта" {
                    dialogue
                        .update(State::RecieveFundingStatus {
                            age,
                            investment_status,
                            instrument: InvestmentInstrument::CryptoCurrency,
                        })
                        .await?;
                } else if instrument == "Вклады" {
                    dialogue
                        .update(State::RecieveFundingStatus {
                            age,
                            investment_status,
                            instrument: InvestmentInstrument::BankDeposits,
                        })
                        .await?;
                }
            } else {
                bot.send_message(msg.chat.id, "Выберите вариант!")
                    .reply_markup(
                        KeyboardMarkup::new(vec![FUNDING_STATUS_SET
                            .iter()
                            .map(|x| KeyboardButton::new(x.to_string()))])
                        .one_time_keyboard(true),
                    )
                    .await?;
            }
        }
    }
    Ok(())
}

async fn recieve_funding_status(
    bot: Bot,
    dialogue: MyDialogue,
    (age, investment_status, instrument): (AgeRange, InvestmentStatus, InvestmentInstrument),
    msg: Message,
) -> HandlerResult {
    let FUNDING_STATUS_SET: HashSet<&str> = HashSet::from_iter(vec![
        "<1 миллиона",
        "1-5 миллионов",
        "5-10 миллионов",
        "Более 10 миллионов",
    ]);
    match msg.text() {
        None => {
            bot.send_message(msg.chat.id, "Выберите вариант!").await?;
        }
        Some(funding_status) => {
            if FUNDING_STATUS_SET.contains(&funding_status) {
                if funding_status == "<1 миллиона" {
                    dialogue
                        .update(State::ReceiveContact {
                            age,
                            investment_status,
                            instrument,
                            funding_status: FundingStatus::LessThanMillion,
                        })
                        .await?;
                } else if funding_status == "1-5 миллионов" {
                    dialogue
                        .update(State::ReceiveContact {
                            age,
                            investment_status,
                            instrument,
                            funding_status: FundingStatus::MillionToFiveMillion,
                        })
                        .await?;
                } else if funding_status == "5-10 миллионов" {
                    dialogue
                        .update(State::ReceiveContact {
                            age,
                            investment_status,
                            instrument,
                            funding_status: FundingStatus::FiveMillionToTenMillion,
                        })
                        .await?;
                } else if funding_status == "Более 10 миллионов" {
                    dialogue
                        .update(State::ReceiveContact {
                            age,
                            investment_status,
                            instrument,
                            funding_status: FundingStatus::TenMillionPlus,
                        })
                        .await?;
                    bot.send_message(
                        msg.chat.id,
                        "Отправьте нам ваш контакт для дальнейшего взаимодействия!",
                    )
                    .reply_markup(
                        KeyboardMarkup::new(vec![vec![
                            KeyboardButton::new("Отправить").request(ButtonRequest::Contact)
                        ]])
                        .one_time_keyboard(true),
                    )
                    .await?;
                } else {
                    bot.send_message(msg.chat.id, "Выберите вариант!").await?;
                }
            }
        }
    }
    Ok(())
}

async fn receive_contact(
    bot: Bot,
    dialogue: MyDialogue,
    (age, investment_status, instrument, funding_status): (
        AgeRange,
        InvestmentStatus,
        InvestmentInstrument,
        FundingStatus,
    ),
    msg: Message,
) -> HandlerResult {
    match msg.contact() {
        None => {
            bot.send_message(msg.chat.id, "Поделитесь контактом!")
                .await?;
        }
        Some(contact) => {
            if contact.phone_number.is_empty() {
                bot.send_message(msg.chat.id, "Не указан номер телефона!")
                    .await?;
            } else {
                dialogue
                    .update(State::End {
                        age: age.clone(),
                        investment_status: investment_status.clone(),
                        instrument: instrument.clone(),
                        funding_status: funding_status.clone(),
                        contact: contact.phone_number.clone(),
                    })
                    .await?;
                bot.send_message(msg.chat.id, "Ожидайте, с вами свяжется наш представитель!")
                    .await?;
                
                let append_string_to_file = append_string_to_file("data.txt", format!("\nВозраст:{}\nСтатус:{}\nИнструмент:{}\nБюджет:{}\nКонтакт:{}", age.clone(), investment_status.clone(), instrument.clone(), funding_status.clone(), contact.phone_number.clone()).as_str()).unwrap();
            }
        }
    }
    Ok(())
}

fn append_string_to_file(path: &str, data: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new().create(true).append(true).open(&path)?;

    // You need to take care of the conversion yourself
    // and can either try to write all data at once
    file.write_all(&data.as_bytes())?;

    // Or try to write as much as possible, but need
    // to take care of the remaining bytes yourself
    let remaining = file.write(&data.as_bytes())?;

    if remaining > 0 {
      // You need to handle the remaining bytes
    }

    Ok(())
}
