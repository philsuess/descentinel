import type { Component } from 'solid-js';
import { ErrorBoundary } from "solid-js";

import { DisplayLatestOverlordCard } from "./latest_overlord_card";
//import { OverlordCard } from "./components/overlord_card";

const App: Component = () => {
  return (
    <>
      <ErrorBoundary fallback={err => err}>
        <DisplayLatestOverlordCard />
      </ErrorBoundary>
    </>
  );
};

export default App;
