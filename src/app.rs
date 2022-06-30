use web_sys::{HtmlCanvasElement, HtmlInputElement, Window};
use yew::events::InputEvent;
use yew::prelude::*;

use crate::helpers::*;
use simulation::{Config, Simulation, SimulationState};
use tracing::{debug, error, info, trace};
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;
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
    SimulationStep,
    NextAgent,
    PreviousAgent,
    SaveBrain,
    DiscardSavedBrain,
}

pub struct App {
    config: Config,
    simulation: Simulation,
    animation_handler: Option<gloo_render::AnimationFrame>,
    window: Window,
    car_canvas: Option<HtmlCanvasElement>,
    network_canvas: Option<HtmlCanvasElement>,
}

impl App {
    fn init_canvases(&mut self) {
        let inner_height = self.window.inner_height().unwrap().as_f64().unwrap() as u32;
        let inner_width = self.window.inner_width().unwrap().as_f64().unwrap();

        let car_canvas = self
            .window
            .document()
            .unwrap()
            .get_element_by_id("carCanvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();

        car_canvas.set_height(inner_height);
        car_canvas.set_width(200);

        self.car_canvas = Some(car_canvas);

        let network_canvas = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("networkCanvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();

        network_canvas.set_height(inner_height);
        network_canvas.set_width((inner_width * 0.4) as u32);
    }

    fn get_car_ctx(&self) -> CanvasRenderingContext2d {
        self.car_canvas
            .as_ref()
            .unwrap()
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap()
    }

    /* fn get_network_ctx(&self) -> CanvasRenderingContext2d {
        self.network_canvas
            .as_ref()
            .unwrap()
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap()
    } */
}

impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let config = Config::load();
        let window = web_sys::window().expect("failed to get window");
        App {
            simulation: Simulation::init(&config),
            config,
            animation_handler: None,
            car_canvas: None,
            network_canvas: None,
            window,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMessage::UpdateLanesCount(v) => {
                parse::<usize>(v).map(|v| self.config.set_lanes_count(v));
            }
            AppMessage::UpdateLaneIndex(v) => {
                parse::<usize>(v).map(|v| self.config.set_lane_index(v));
            }
            AppMessage::UpdateCarsCount(v) => {
                parse::<usize>(v).map(|v| self.config.set_cars_count(v));
            }
            AppMessage::UpdateRaysCount(v) => {
                parse::<usize>(v).map(|v| self.config.set_rays_count(v));
            }
            AppMessage::UpdateRaysLength(v) => {
                parse::<f64>(v).map(|v| self.config.set_rays_length(v));
            }
            AppMessage::UpdateRaysSpread(v) => {
                parse::<f64>(v).map(|v| self.config.set_rays_spread(v));
            }
            AppMessage::UpdateHiddenLayers(v) => self.config.parse_and_set_hidden_layers(&v),
            AppMessage::UpdateMutationRate(v) => {
                parse::<f64>(v).map(|v| self.config.set_mutation_rate(v));
            }
            AppMessage::RunSimulation => {
                tracing::trace!("received request to run simulation");
                let l = ctx.link().clone();
                self.simulation = Simulation::init(&self.config);
                self.simulation.run();
                let animation_handler = gloo_render::request_animation_frame(move |_v| {
                    l.send_message(AppMessage::SimulationStep)
                });

                tracing::trace!(?animation_handler, "got animation handler");

                self.animation_handler = Some(animation_handler);
            }
            AppMessage::StartPauseSimulation => match self.simulation.state {
                SimulationState::Paused => self.simulation.run(),
                SimulationState::Running => self.simulation.pause(),
                _ => (),
            },
            AppMessage::StopSimulation => {
                let handler = self.animation_handler.take();
                drop(handler)
            }
            AppMessage::SimulationStep => {
                let (_, network_ctx) = get_cvs_ctx(self);
                tracing::trace!("requested simulation step");
                let l = ctx.link().clone();
                self.simulation
                    .step(&self.get_car_ctx(), &network_ctx, 700.);

                let animation_handler = gloo_render::request_animation_frame(move |_v| {
                    l.send_message(AppMessage::SimulationStep)
                });

                self.animation_handler = Some(animation_handler);
            }
            AppMessage::NextAgent => todo!("not yet implemented, NextAgent"),
            AppMessage::PreviousAgent => todo!("not yet implemented, PreviousAgent"),
            AppMessage::SaveBrain => todo!("not yet implemented, SaveBrain"),
            AppMessage::DiscardSavedBrain => todo!("not yet implemented, DiscardSavedBrain"),
        }
        true
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
                            id="runBtn"
                            onclick = { ctx.link().callback(|_| AppMessage::RunSimulation) }>
                                {"Run"}
                        </button>
                        <button
                            id="resetFocusBtn">
                                {"Reset Focus"}
                        </button>
                    </div>

                </div>

                <div id="middleSection">
                    <canvas id="carCanvas"/>
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

fn get_cvs_ctx(app: &mut App) -> (CanvasRenderingContext2d, CanvasRenderingContext2d) {
    let window = web_sys::window().expect("no fcking window found");
    let inner_height = window.inner_height().unwrap().as_f64().unwrap() as u32;

    let car_canvas = window
        .document()
        .unwrap()
        .get_element_by_id("carCanvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    car_canvas.set_height(inner_height);
    car_canvas.set_width(200);

    if app.car_canvas.is_none() {
        tracing::trace!("adding new canvas to app");
        app.car_canvas = Some(car_canvas);
    }

    let car_cvs_ctx = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("carCanvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap()
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let network_cvs_ctx = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("networkCanvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap()
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    (car_cvs_ctx, network_cvs_ctx)
}
