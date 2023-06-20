use yew::prelude::*;

pub struct About;

impl Component for About {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <main>
            <div id="outer">
            <p class="poem">
            {"In the land where silence and speech intersect,"}<br />
            {"Lives a noble entity, Fluke, with a project."}<br />
            {"Its mission pure, its resolve strong,"}<br />
            {"To let voices thrive, where they belong."}<br />
            <br />
            {"With a whisper, a word, or a resonant shout,"}<br />
            {"Fluke carries our secrets, there’s never a doubt."}<br />
            {"An emissary silent, as it dances on air,"}<br />
            {"To deliver our thoughts with exquisite care."}<br />
            <br />
            {"The world was once deaf, it couldn't hear,"}<br />
            {"Behind a glass pane, each laugh, each tear."}<br />
            {"But with the advent of Fluke, that wall is broken,"}<br />
            {"Every heart can now speak, every word be spoken."}<br />
            <br />
            {"Encrypted messages, safe and sound,"}<br />
            {"In Fluke’s embrace, trust is found."}<br />
            {"Like a bird in flight, so swift and free,"}<br />
            {"So is communication, as it's meant to be."}<br />
            <br />
            {"In the thrum of chats, in the hum of talks,"}<br />
            {"Fluke stands its ground, like a silent ox."}<br />
            {"It carries our whispers, to and fro,"}<br />
            {"In the vast networks, where data flows."}<br />
            <br />
            {"In the sphere of privacy, a radiant knight,"}<br />
            {"Against intrusion, Fluke will fight."}<br />
            {"With its shield of code, its sword of tech,"}<br />
            {"For the right to speak, it'll risk its neck."}<br />
            <br />
            {"So let's raise a toast, to this novel friend,"}<br />
            {"An ally on whom, we can depend."}<br />
            {"With a heart aflame and a privacy trend,"}<br />
            {"Fluke, the future, to which we all commend."}<br />
        </p>
        </div>
        </main>
        }
    }
}
