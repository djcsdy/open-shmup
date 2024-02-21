import {start} from "open-shmup";
import * as gameUrl from "./atomsmash.openshmup";

void fetch(gameUrl)
    .then(async response => response.blob())
    .then(async blob => blob.arrayBuffer())
    .then(async buffer => new Uint8Array(buffer))
    .then(async game => start(game));
