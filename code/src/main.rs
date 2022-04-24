/*
 * Generate Discord-friendly timestamps in a webapp.
 * https://github.com/monktype/timestamps
 *
 * Main Rust wasm app code.
 */

use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::{
    events::Event,
    html,
    Component, Context, Html,
};
use chrono::TimeZone;

enum Msg {
    UpdateTime(String),
    UpdateFormat(String),
}

enum TimeFormat {
    ShortDate, // "23/04/2022"
    WrittenDate, // "23 April 2022"
    WrittenDateTime, // "23 April 2022 18:00"
    TimeOnly, // "18:00"
    TimeSeconds, // "18:00:00"
    Relative, // "In 9 minutes"
    FullDateTime, // "Saturday, 23 April 2022 18:00"
}

struct App {
    local_time: chrono::DateTime<chrono::Local>,
    input_local_time: String,
    result_timestamp: i64,
    result_format: TimeFormat,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let current_local_time = chrono::Local::now(); // This is probably the best place for this.
        Self {
            local_time: current_local_time,
            input_local_time: current_local_time.format("%Y-%m-%dT%H:%M").to_string(),
            result_timestamp: current_local_time.timestamp(),
            result_format: TimeFormat::FullDateTime,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateTime(value) => {
                self.input_local_time = value;
                let parsed_input_local_time = chrono::NaiveDateTime::parse_from_str(&(self.input_local_time), "%Y-%m-%dT%H:%M").unwrap();
                self.result_timestamp = self.local_time.timezone().from_local_datetime(&parsed_input_local_time).unwrap().timestamp();
                true
            },
            Msg::UpdateFormat(value) => {
                self.result_format = match value.as_str() {
                    "shortdate" => TimeFormat::ShortDate,
                    "writtendate" => TimeFormat::WrittenDate,
                    "writtendatetime" => TimeFormat::WrittenDateTime,
                    "timeonly" => TimeFormat::TimeOnly,
                    "timeseconds" => TimeFormat::TimeSeconds,
                    "relative" => TimeFormat::Relative,
                    "fulldatetime" => TimeFormat::FullDateTime,
                    _ => TimeFormat::FullDateTime,
                };
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let timeinput_change = link.callback(|event: Event| {
            let target: EventTarget = event.target().expect("Was there no target?");
            Msg::UpdateTime(target.unchecked_into::<HtmlInputElement>().value())
        });
        let format_change = link.callback(|event: Event| {
            let target: EventTarget = event.target().expect("Was there no target?");
            Msg::UpdateFormat(target.unchecked_into::<HtmlInputElement>().value())
        });
        html! {
            <div>
            <div class="centered">
                <div>
                    <h2>{"Discord Timestamp Tool"}</h2>
                    <br/><br/><p>{"Select the time: "} <input type="datetime-local" onchange={timeinput_change} value={ self.input_local_time.clone() }/></p> 
                    <p><font size="-2"><i>{format!("Use your local timezone. This tool uses your browser to determine your timezone. Currently: {}", self.local_time.format("UTC%z"))}</i></font></p>
                    <p>{"Select the time format: "} <select onchange={format_change}>
                        <option value="shortdate">{"23/04/2022"}</option>
                        <option value="writtendate">{"23 April 2022"}</option>
                        <option value="writtendatetime">{"23 April 2022 18:00"}</option>
                        <option value="timeonly">{"18:00"}</option>
                        <option value="timeseconds">{"18:00:00"}</option>
                        <option value="relative">{"In 9 minutes"}</option>
                        <option value="fulldatetime">{"Saturday, 23 April 2022 18:00"}</option>
                    </select></p>
                    <p><font size="-2"><i>{"The format drop-down options show example times only. Discord may localize each in a different way."}</i></font></p>
                    <p><br/>{"Discord-friendly timestamp:"}
                    <br/><div class="result"><font size="+2"><b>{format!("<t:{}:{}>", self.result_timestamp, match self.result_format {
                        TimeFormat::ShortDate => "d",
                        TimeFormat::WrittenDate => "D",
                        TimeFormat::WrittenDateTime => "f",
                        TimeFormat::TimeOnly => "t",
                        TimeFormat::TimeSeconds => "T",
                        TimeFormat::Relative => "R",
                        TimeFormat::FullDateTime => "F",
                    })}</b></font></div></p>
                </div>
            </div>
            <div class="footer"><font size="-2">{"Monktype on "}<a href="https://github.com/Monktype">{"GitHub"}</a>{" and "}<a href="https://www.twitch.tv/monktype">{"Twitch"}</a>{"!"}</font></div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
