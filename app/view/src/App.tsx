import type { Component } from 'solid-js';
import { ErrorBoundary } from "solid-js";

import { DisplayLatestGameRoomImage } from "./latest_game_room_image";
import {DisplayLatestOverlordCard} from "./latest_overlord_card";

const App: Component = () => {
  return (
    <>
      <ErrorBoundary fallback={err => err}>
        <DisplayLatestOverlordCard/>
        <DisplayLatestGameRoomImage />
      </ErrorBoundary>
    </>
  );
};

export default App;
