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

    // console.log('on message: \n ' + JSON.stringify(event.data));

    await p;
    let e = new MessageEvent("external_message", { data: event.data });
    window.dispatchEvent(e);

});

// set state
if (!vscode) { vscode = acquireVsCodeApi(); }
function setState(str){
    console.log('state to set: ' + str);
    let obj = JSON.parse(str);
    vscode.setState(obj);
}



// // const bqTableTagName = 'BQ-TABLE';
// const bqQueryTagName = 'BQ-QUERY';

// let bqQueryOnRender = function (event) {

//     const bqTable = event.target;
//     const jobId = bqTable.getAttribute('job_id');
//     const projectId = bqTable.getAttribute('project_id');
//     const location = bqTable.getAttribute('location');

//     console.log('bqQueryOnRender');

//     if (!vscode) { vscode = acquireVsCodeApi(); }
//     vscode.setState({ jobId, projectId, location });

//     console.log('state set: ', JSON.stringify({ jobId, projectId, location }));


// };

// let mutationObserver = new MutationObserver(function (mutations) {

//     for (let index = 0; index < mutations.length; index++) {
//         const mutation = mutations[index];

//         for (let index = 0; index < mutation.addedNodes.length; index++) {
//             const node = mutation.addedNodes[index];

//             const tagName = node.tagName;

//             if (tagName === bqQueryTagName) {
//                 node.addEventListener('render_table', bqQueryOnRender);
//                 console.log('node added', tagName, ' and event listener added');
//             }
//             // else {

//             //     if (tagName === 'DIV' && node.getAttribute('onAttributeMutation')) {
//             //         console.log('onAttributeMutation: ', tagName);
//             //         attributesMutationObserver.observe(node, { attributes: true, childList: false, characterData: false, subtree: false });
//             //     }

//             // }
//         }
//     }
// });

// mutationObserver.observe(document, { attributes: false, childList: true, characterData: false, subtree: true });


// https://developer.mozilla.org/en-US/docs/Web/API/Intersection_Observer_API
// IntersectionObserver

let intersectionObserver = new IntersectionObserver((entries, observer) => {
    entries.forEach((entry) => {

        // console.log('boundingClientRect: ' + entry.boundingClientRect);
        // console.log('intersectionRatio:' + entry.intersectionRatio);
        // console.log('intersectionRect:' + entry.intersectionRect);
        // console.log('isIntersecting:' + entry.isIntersecting);
        // console.log('rootBounds:' + entry.rootBounds);
        // console.log('target:' + entry.target);
        // console.log('time:' + entry.time);

        // const tagName = entry.target.tagName;
        // console.log('tagName: ' + tagName);

        if (entry.isIntersecting) {
            console.log('element is now in visible scope: ' + entry.target.id);
            entry.target.dispatchEvent(new CustomEvent("element_intersected", { detail: entry }));
        }
        });
    }
    , {
        root: document, //.querySelector("#q1"),
        rootMargin: "0px",
        threshold: 1.0,
    });

// function observe(elementId) {
//     let element = document.getElementById(elementId);
//     console.log("element with id '" + elementId + "' is being observed");
//     intersectionObserver.observe(element);
// }

function observeElement(element) {
    intersectionObserver.observe(element);
}

document.observeElement = observeElement;
document.setState = setState;