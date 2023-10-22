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

//     // //of the type ResultsGridRenderRequest
//     // const message = event.data;

//     // const q1 = document.getElementById('q1');
//     // q1.setAttribute("startIndex", message.startIndex);
//     // q1.setAttribute("maxResults", message.maxResults);
//     // // q1.setAttribute("jobIndex", message.jobIndex);
//     // q1.setAttribute("openInTabVisible", message.openInTabVisible);
//     // q1.setAttribute("projectId", message.jobReferences[message.jobIndex].projectId);
//     // q1.setAttribute("jobId", message.jobReferences[message.jobIndex].jobId);
//     // q1.setAttribute("location", message.jobReferences[message.jobIndex].location);
//     // q1.setAttribute("token", message.token);
//     // q1.dispatchEvent(new Event("render_table"));

});