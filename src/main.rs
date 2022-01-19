use iced::{executor, Application, Command, Element, Text,Settings};
use std::path::PathBuf;

struct GUI;
fn main (){
    GUI::run(Settings{
        default_font:Some(include_bytes!("../font/NotoSansJP-Regular.otf")),
        ..Settings::default() // .. はまとめて全部処理？
    });
}

impl Application for GUI {
    type Executor = executor::Default;
    type Message = ();
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (GUI, Command::none())
    }

    fn title(&self) -> String {
        String::from("Hello Iced!")
    }

    fn update(&mut self, message: Self::Message, clipboard: &mut iced::Clipboard) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        Text::new("Test Iced!").into()
    }
}


/* 
use iced::{
    executor, Align, Application, Clipboard, Column, Command, Container, Element, Image, Length,
    Settings, Subscription, Text,
};
use std::path::PathBuf;

pub fn main() -> iced::Result {
    // フォントを指定しつつ実行する。
    Events::run(Settings {
        default_font: Some(include_bytes!("../font/NotoSansJP-Regular.otf")),
        ..Settings::default()
    })
}

// メインとなる構造体。アプリで保持する状態を変数にする。
#[derive(Debug, Default)]
struct Events {
    path: PathBuf,
}

// 何らかの変更があったときに飛ぶメッセージ。今回はイベント発生のみ。
#[derive(Debug, Clone)]
enum Message {
    EventOccurred(iced_native::Event),
}

impl Application for Events {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Events, Command<Message>) {
        (Events::default(), Command::none())
    }

    // ウィンドウのタイトル。状態に合わせた動的な生成も可。
    fn title(&self) -> String {
        String::from("Image Viewer")
    }

    // 何らかの変更があったときに呼び出される。
    // 発生した事柄はenum（今回の場合はMessage）として伝えられる。
    // Icedのバージョン0.3から引数にClipboardが増えたが、使わないので無視。
    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        // ファイルがドロップされたときに、アプリの状態を変更する。
        // Eventのenumの中に、イベントの内容（別のEventのenum）とか、
        // 今回のFileDroppedではファイルパスが含まれたりする。
        match message {
            Message::EventOccurred(event) => {
                if let iced_native::event::Event::Window(we) = event {
                    if let iced_native::window::Event::FileDropped(path) = we {
                        self.path = path;
                    }
                }
            }
        };

        Command::none()
    }

    // イベントが発生したときに呼び出される。マウス操作、ウィンドウ関係、キーボード操作等。
    // 何らかのSubscriptionを返すことで、update()が実行される。
    fn subscription(&self) -> Subscription<Message> {
        iced_native::subscription::events().map(Message::EventOccurred)
    }

    // 表示されるGUIを生成する。
    fn view(&mut self) -> Element<Message> {
        // ファイルパス表示部
        let mut p = self.path.to_str().unwrap_or("").to_string();
        if p.is_empty() {
            p = String::from("画像ファイルをウィンドウにドロップしてね。");
        }
        let path = Container::new(Text::new(p).size(20)).padding(4);

        // 画像表示部
        let image = Container::new(
            Image::new(self.path.clone())
                .width(Length::FillPortion(2))
                .height(Length::FillPortion(3)),
        )
        .height(Length::FillPortion(5))
        .width(Length::FillPortion(5))
        .align_x(Align::Center)
        .align_y(Align::Center);

        let content = Column::new()
            .width(Length::Fill)
            .align_items(Align::Start)
            .push(path)
            .push(image);

        Container::new(content)
            .width(Length::FillPortion(2))
            .height(Length::FillPortion(2))
            .into()
    }
}
*/



