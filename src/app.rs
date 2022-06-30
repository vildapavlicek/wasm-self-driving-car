use std::ops::Deref;
use tap::tap::Tap;
use web_sys::HtmlInputElement;
use yew::events::{Event, InputEvent};
use yew::prelude::*;

const LOCAL_STORAGE_BRAIN_KEY: &str = "wasm-self-driving-car.brain.save";
const LOCAL_STORAGE_CONFIG_KEY: &str = "wasm-self-driving-car.config.save";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Config {
    pub lanes_count: usize,
    pub lane_index: usize,
    pub cars_count: usize,
    pub rays_count: usize,
    pub rays_length: f64,
    pub rays_spread: f64,
    pub hidden_layers: Vec<usize>,
    pub mutation_rate: f64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            lanes_count: 3,
            lane_index: 1,
            cars_count: 100,
            rays_count: 5,
            rays_length: 120.,
            rays_spread: 2.,
            hidden_layers: vec![6],
            mutation_rate: 0.2,
        }
    }
}

impl Config {
    fn set_lanes_count(&mut self, value: &str) {
        match value.parse::<usize>() {
            Ok(v) => self.lanes_count = v,
            Err(err) => tracing::error!(field = "lanes_count", %err, "failed to update config"),
        }
    }

    fn set_lane_index(&mut self, value: &str) {
        match value.parse::<usize>() {
            Ok(v) => self.lane_index = v,
            Err(err) => tracing::error!(field = "lane_index", %err, "failed to update config"),
        }
    }

    fn set_cars_count(&mut self, value: &str) {
        match value.parse::<usize>() {
            Ok(v) => self.cars_count = v,
            Err(err) => tracing::error!(field = "cars_count", %err, "failed to update config"),
        }
    }

    fn set_rays_count(&mut self, value: &str) {
        match value.parse::<usize>() {
            Ok(v) => self.rays_count = v,
            Err(err) => tracing::error!(field = "rays_count", %err, "failed to update config"),
        }
    }

    fn set_rays_length(&mut self, value: &str) {
        match value.parse::<f64>() {
            Ok(v) => self.rays_length = v,
            Err(err) => tracing::error!(field = "rays_length", %err, "failed to update config"),
        }
    }

    fn set_rays_spread(&mut self, value: &str) {
        match value.parse::<f64>() {
            Ok(v) => self.rays_spread = v,
            Err(err) => tracing::error!(field = "rays_spread", %err, "failed to update config"),
        }
    }

    fn set_hidden_layers(&mut self, value: &str) {
        let hidden_layers = value
            .split(',')
            .filter_map(|v| v.parse::<usize>().ok())
            .collect::<Vec<usize>>();
        if hidden_layers.is_empty() {
            tracing::error!("failed to parse hidden layers");
            return;
        }

        self.hidden_layers = hidden_layers;
    }

    fn set_mutation_rate(&mut self, value: &str) {
        match value.parse::<f64>() {
            Ok(v) => self.mutation_rate = v,
            Err(err) => tracing::error!(field = "mutation_rate", %err, "failed to update config"),
        }
    }

    fn load() -> Self {
        use gloo_storage::Storage;

        gloo_storage::LocalStorage::get(LOCAL_STORAGE_CONFIG_KEY)
            .tap(|config| tracing::info!(?config, "local storage config"))
            .unwrap_or_default()
    }

    fn save(&self) {
        use gloo_storage::Storage;

        if let Err(err) = gloo_storage::LocalStorage::set(LOCAL_STORAGE_CONFIG_KEY, self) {
            tracing::error!(config = ?self, %err, "failed to save config to local storage");
        }
    }

    fn hidden_layers(&self) -> String {
        use std::io::Write;
        let mut buffer = Vec::with_capacity(self.hidden_layers.capacity());
        self.hidden_layers.iter().for_each(|n| {
            write!(&mut buffer, "{},", n).ok();
        });
        buffer.pop();

        String::from_utf8(buffer).expect("failed to write hidden layers as string")
    }
}

pub enum AppMessage {
    UpdateLanesCount(String),
    UpdateLaneIndex(String),
    UpdateCarsCount(String),
    UpdateRaysCount(String),
    UpdateRaysLength(String),
    UpdateRaysSpread(String),
    UpdateHiddenLayers(String),
    UpdateMutationRate(String),
    RunSimulation,
    StartPauseSimulation,
    StopSimulation,
    NextAgent,
    PreviousAgent,
    SaveBrain,
    DiscardSavedBrain,
}

pub struct App {
    config: Config,
}

impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App {
            config: Config::load(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMessage::UpdateLanesCount(v) => self.config.set_lanes_count(&v),
            AppMessage::UpdateLaneIndex(v) => self.config.set_lane_index(&v),
            AppMessage::UpdateCarsCount(v) => self.config.set_cars_count(&v),
            AppMessage::UpdateRaysCount(v) => self.config.set_rays_count(&v),
            AppMessage::UpdateRaysLength(v) => self.config.set_rays_length(&v),
            AppMessage::UpdateRaysSpread(v) => self.config.set_rays_spread(&v),
            AppMessage::UpdateHiddenLayers(v) => self.config.set_hidden_layers(&v),
            AppMessage::UpdateMutationRate(v) => self.config.set_mutation_rate(&v),
            AppMessage::RunSimulation => todo!("implement run simulation"),
            AppMessage::StartPauseSimulation => todo!("implement pause/run simulation"),
            AppMessage::StopSimulation => todo!("not yet implemented, StopSimulation"),
            AppMessage::NextAgent => todo!("not yet implemented, NextAgent"),
            AppMessage::PreviousAgent => todo!("not yet implemented, PreviousAgent"),
            AppMessage::SaveBrain => todo!("not yet implemented, SaveBrain"),
            AppMessage::DiscardSavedBrain => todo!("not yet implemented, DiscardSavedBrain"),
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div id = "leftSection">
                    <form id="settingsForm">
                        <label
                            class="settingsFormLabel">
                                { "lanes count" }
                        </label>
                        <br/>
                        <input
                            id="lanesCountInput"
                            type="text"
                            class="settingsFormInput"
                            value = { self.config.lanes_count.to_string() }
                            oninput = { ctx.link().callback(move |e: InputEvent|
                                AppMessage::UpdateLanesCount(e.target_unchecked_into::<HtmlInputElement>().value())
                            )}
                        />
                        <br/>
                        <label
                            class="settingsFormLabel">
                                { "lane index" }
                        </label>
                        <br/>
                        <input
                            id="laneIndexInput"
                            type="text"
                            class="settingsFormInput"
                            value = { self.config.lane_index.to_string() }
                            oninput = { ctx.link().callback(move |e: InputEvent|
                                AppMessage::UpdateLaneIndex(e.target_unchecked_into::<HtmlInputElement>().value())
                            )}
                        />
                        <br/>
                        <label
                            class="settingsFormLabel">
                                { "cars count" }
                        </label>
                        <br/>
                        <input
                            id="carsCountInput"
                            type="text"
                            class="settingsFormInput"
                            value = { self.config.cars_count.to_string() }
                            oninput = { ctx.link().callback(move |e: InputEvent|
                                AppMessage::UpdateCarsCount(e.target_unchecked_into::<HtmlInputElement>().value())
                            )}
                        />
                        <br/>
                        <label
                            class="settingsFormLabel">
                                { "rays count" }
                        </label>
                        <br/>
                        <input
                            id="raysCountInput"
                            type="text"
                            class="settingsFormInput"
                            value = { self.config.rays_count.to_string() }
                            oninput = { ctx.link().callback(move |e: InputEvent|
                                AppMessage::UpdateRaysCount(e.target_unchecked_into::<HtmlInputElement>().value())
                            )}
                        />
                        <br/>
                        <label
                            class="settingsFormLabel">
                                { "rays length" }
                        </label>
                        <br/>
                        <input
                            id="raysLengthInput"
                            type="text"
                            class="settingsFormInput"
                            value = { self.config.rays_length.to_string() }
                            oninput = { ctx.link().callback(move |e: InputEvent|
                                AppMessage::UpdateRaysLength(e.target_unchecked_into::<HtmlInputElement>().value())
                            )}
                        />
                        <br/>
                        <label
                            class="settingsFormLabel">
                                { "rays spread" }
                        </label>
                        <br/>
                        <input
                            id="raysSpread"
                            type="text"
                            class="settingsFormInput"
                            value = { self.config.rays_spread.to_string() }
                            oninput = { ctx.link().callback(move |e: InputEvent|
                                AppMessage::UpdateRaysSpread(e.target_unchecked_into::<HtmlInputElement>().value())
                            )}
                        />
                        <br/>
                        <label
                            class="settingsFormLabel">
                            { "hidden layers" }
                        </label>
                        <br/>
                        <input
                            id="hiddenLayersInput"
                            type="text"
                            class="settingsFormInput"
                            value = { self.config.hidden_layers() }
                            oninput = { ctx.link().callback(move |e: InputEvent|
                                AppMessage::UpdateHiddenLayers(e.target_unchecked_into::<HtmlInputElement>().value())
                            )}
                        />
                        <br/>
                        <label
                            class="settingsFormLabel">
                                { "mutation rate" }
                        </label>
                        <br/>
                        <input
                            id="mutationRateInput"
                            type="text"
                            class="settingsFormInput"
                            value = { self.config.mutation_rate.to_string() }
                            oninput = { ctx.link().callback(move |e: InputEvent|
                                AppMessage::UpdateMutationRate(e.target_unchecked_into::<HtmlInputElement>().value())
                            )}
                        />
                    </form>


                    <div id = "menu">
                        <form id = "horizontalSpawner">
                        <label class="settingsFormLabel">{"lane ids"}</label>
                        <input
                            id="horizontalSpawnerLaneIdInput"
                            type="text"
                            class="settingsFormInput" />
                        </form>

                        <button
                            id="horizontalSpawnBtn">
                                {"Spawn Horizontal"}
                        </button>

                        <button
                            id="trainingTrafficBtn">
                                {"Training Traffic"}
                        </button>
                        <button
                            id="easyTestBtn">
                                {"Easy Test"}
                        </button>
                        <button
                            id="mediumTestBtn">
                                {"Medium Test"}
                        </button>
                        <button
                            id="hardTestBtn">
                                {"Hard Test"}
                        </button>

                        <button
                            id="runBtn">
                                {"Run"}
                        </button>
                        <button
                            id="resetFocusBtn">
                                {"Reset Focus"}
                        </button>
                    </div>

                </div>

                <div id="middleSection">
                    <canvas id="carCanvas" />
                    <div id="verticalButtons">
                        <span class="emojiHeader" id="topBorder"> {"🧬"}</span>
                        <button
                            id="startPause"
                            onclick = {ctx.link().callback(|_| AppMessage::StartPauseSimulation)}>
                            {"⏯️"}
                        </button>
                        <button
                            id="stop"
                            onclick = {ctx.link().callback(|_| AppMessage::StopSimulation)}>
                            {"🛑"}
                        </button>
                        <span class="emojiHeader" id="topBottomBorder">{"🧠"}</span>
                        <button
                            id="nextAgentBtn"
                            onclick = {ctx.link().callback(|_| AppMessage::NextAgent)}>
                            {"➡️"}
                        </button>
                        <button
                            id="previousAgentBtn"
                            onclick = {ctx.link().callback(|_| AppMessage::PreviousAgent)}>
                            {"⬅️"}
                        </button>
                        <button
                            id="save"
                            onclick = {ctx.link().callback(|_| AppMessage::SaveBrain)}>
                            {"💾"}
                        </button>
                        <button
                            id="discard"
                            onclick = {ctx.link().callback(|_| AppMessage::DiscardSavedBrain)}>
                            {"🗑️"}
                        </button>
                    </div>
                </div>

                <div id="rightSection">
                    <canvas id="networkCanvas" />
                </div>
            </>
        }
    }
}
