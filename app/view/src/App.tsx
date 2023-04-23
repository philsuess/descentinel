import type { Component } from 'solid-js';
import { ErrorBoundary } from "solid-js";

import { DisplayOverlordCard } from "./overlord_card";

const App: Component = () => {
  return (
    <>
      <ErrorBoundary fallback={err => err}>
        <DisplayOverlordCard />
      </ErrorBoundary>
    </>
  );
};

export default App;
