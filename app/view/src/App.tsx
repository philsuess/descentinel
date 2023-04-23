import type { Component } from 'solid-js';

import { DisplayGameRoom } from "./game_room";

import logo from './logo.svg';
import styles from './App.module.css';

const header_from_template = (<div class={styles.App}>
  <header class={styles.header}>
    <img src={logo} class={styles.logo} alt="logo" />
    <p>
      Edit <code>src/App.tsx</code> and save to reload.
    </p>
    <a
      class={styles.link}
      href="https://github.com/solidjs/solid"
      target="_blank"
      rel="noopener noreferrer"
    >
      Learn Solid
    </a>
  </header>
</div>)

const App: Component = () => {
  return (
    <>
      <DisplayGameRoom />
    </>
  );
};

export default App;
