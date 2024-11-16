async function execute_luau(LuauCode) {
    const LuauWasm = await import('./target/wasm32-unknown-emscripten/release/webluau.js');
    const ModuleData = {
        print(str) {
            console.log(str);
        },
        printErr(str) {
            console.error(str);
        }   
    };
    const LuauModule = await LuauWasm.default(ModuleData);

    const LuauInstance = LuauModule.ccall('lua_new', 'number', [], []);
    const ExecLuau = LuauModule.cwrap('lua_execute', null, [ 'number', 'string' ]);

    ExecLuau(LuauInstance, LuauCode);
}

class Luau extends HTMLElement {
    constructor() {
        super();
        this.LuauCode = this.textContent.trim();
        this.style.display = 'none';
        this.#initialize();
    }

    #initialize() {
        execute_luau(this.LuauCode);
    }
}

customElements.define('lua-u', Luau);