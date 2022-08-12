import "./index.scss";

import { render } from "preact";

import { App } from "./app";

render(<App />, document.getElementById("container") as HTMLElement);
