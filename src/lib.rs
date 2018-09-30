extern crate stdweb;
#[macro_use]
extern crate yew;

use yew::prelude::*;
use yew::services::ConsoleService;

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
            value: vec![vec![Cell::Empty; 6]; 7],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Play(x) => {
                self.value[5][x] = Cell::Player1;
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
            <div>
                <table>
                    { for self.value.iter().enumerate().map(|(y, row)| view_row(row, y)) }
                </table>
            </div>
        }
    }
}

fn view_row(row: &Vec<Cell>, y: usize) -> Html<Model> {
    html! {
        <tr>
            { for row.iter().enumerate().map(|(x, cell)| view_cell(cell, x, y)) }
        </tr>
    }
}

fn view_cell(cell: &Cell, x: usize, y: usize) -> Html<Model> {
    let cell_status = match cell {
        Cell::Empty => "cell-empty",
        Cell::Player1 => "cell-player1",
        Cell::Player2 => "cell-player2"
    };
    html! {
        <td>
            <div class=("cell", cell_status), onclick=|_| Msg::Play(x),>
                {x} {y}
            </div>
        </td>
    }
}
