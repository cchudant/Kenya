extern crate stdweb;
#[macro_use]
extern crate yew;

use yew::prelude::*;
use yew::services::ConsoleService;

const SIZE: (usize, usize) = (6, 7);

#[derive(Clone)]
pub enum Cell {
    Empty,
    Player1,
    Player2
}

pub struct Model {
    console: ConsoleService,
    value: Vec<Vec<Cell>>,
}

pub enum Msg {
    Play(usize),
    Clear,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            console: ConsoleService::new(),
            value: vec![vec![Cell::Empty; SIZE.1]; SIZE.0],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Play(x) => {
                self.value[0][x] = Cell::Player2;
                self.value[1][x] = Cell::Player2;
                self.value[2][x] = Cell::Player2;
                self.value[3][x] = Cell::Player2;
                self.value[4][x] = Cell::Player2;
                self.value[5][x] = Cell::Player2;
                self.console.log("Play");
            },
            Msg::Clear => {
                self.value.clear();
                self.console.log("Clear");
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="main",>
                <div class="centered",>
                    <div class="board",>
                        <img src="c4board.svg",/>
                    </div>
                    <table class="columns",><tr>
                        { for (0..SIZE.1).map(|i| html! {
                            <td class="column", onclick=|_| Msg::Play(i),></td>
                        }) }
                    </tr></table>
                    <div class="pieces",>
                        { self.view_pieces() }
                    </div>
                </div>
            </div>
        }
    }
}

impl Model {
    fn view_pieces(&self) -> Html<Self> {
        let mut vec = vec![];
        for (y, v) in self.value.iter().enumerate() {
            for (x, e) in v.iter().enumerate() {
                if let Some((class, svg)) = match e {
                    Cell::Empty => None,
                    Cell::Player1 => Some(("cell-player1", "c4red.svg")),
                    Cell::Player2 => Some(("cell-player2", "c4yellow.svg"))
                } {
                    vec.push(html! {
                        <div class=("cell", class), style=format!("left: {}px; bottom: {}px", 109.0 + 92.0*(x as f64), 12.0 + 92.0*(y as f64)),>
                            <img src=svg,/>
                        </div>
                    })
                }
            }
        }

        html! {
            { for vec }
        }
    }
}
