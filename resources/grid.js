// https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm#using_the_package_on_the_web

import init, { get_web_components_list, register_custom_element, on_window_message_received } from "../dist/grid_render.js";

const p = init().then(() => {

    let webComponentsList = get_web_components_list();

    for (let index = 0; index < webComponentsList.length; index++) {
        const elementName = webComponentsList[index];

        window.customElements.define(elementName, class extends HTMLElement {
            connectedCallback() {
                // console.log(elementName);
                register_custom_element(elementName, this);
            }
        });
    }

});

window.addEventListener('external_message', on_window_message_received);

// pass along the messsage but guarantees that the custom elements are loaded
window.addEventListener('message', async event => {

    await p;
    let e = new MessageEvent("external_message", { data: event.data });
    window.dispatchEvent(e);

});

let observer = new IntersectionObserver((entries, observer) => {
    entries.forEach((entry) => {

        console.log(entry);
      // Each entry describes an intersection change for one observed
      // target element:
      //   entry.boundingClientRect
      //   entry.intersectionRatio
      //   entry.intersectionRect
      //   entry.isIntersecting
      //   entry.rootBounds
      //   entry.target
      //   entry.time
    });
  }, {
    root: document.querySelector("#q1"),
    rootMargin: "0px",
    threshold: 1.0,
});

function observe(elementId) {
    let element = document.getElementById(elementId);
    console.log("element with id '" + elementId + "' is being observed");
    observer.observe(element);
}

document.observe = observe;