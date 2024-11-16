use mlua::prelude::*;
use std::{ffi::CStr, os::raw::c_char, sync::{Arc, Mutex}};
use emscripten_val::*;

fn lua_alert(_: &Lua, message: String) -> LuaResult<()> {
    let window = Val::global("window");
    let message_str = message.as_str();
    window.call("alert", argv![ message_str ]);
    Ok(())
}

fn lua_write(_: &Lua, text: String) -> LuaResult<()> {
    let document = Val::global("document");
    let text_str = text.as_str();
    document.call("write", argv![ text_str ]);
    Ok(())
}


fn lua_on_click(element: Val, callback: LuaFunction) {
    element.set(
        &"onclick",
        &Val::from_fn1(move |_ev| {
            callback.call::<()>(()).unwrap();
            ().into()
        })
    );
}

fn element_to_table(lua: &Lua, element: Val) -> LuaTable {
    let element_table = lua.create_table().unwrap();

    element_table
    .set("content", element.get(&"textContent").as_string())
    .unwrap();
    element_table
    .set("inner_html", element.get(&"innerHTML").as_string())
    .unwrap();
    element_table
    .set("outer_html", element.get(&"outerHTML").as_string())
    .unwrap();

    let element_arc = Arc::new(Mutex::new(element));

    element_table
    .set(
        "on_click",
        lua.create_function(move |_: &_, callback: LuaFunction| {
            let element_lock = element_arc.lock().unwrap();
            lua_on_click(element_lock.clone(), callback);
            Ok(())
        })
        .unwrap(),
    )
    .unwrap();

    element_table
}

fn lua_query_selector(lua: &Lua, query: String) -> LuaResult<LuaTable> {
    let document = Val::global("document");
    let query_str = query.as_str();
    let element = document.call("querySelector", argv![ query_str ]);

    if element.is_null() {
        return Ok(lua.create_table().unwrap());
    }

    let element_table = element_to_table(lua, element);

    Ok(element_table)
}

#[no_mangle]
pub extern "C" fn lua_new() -> *mut Lua {
    let lua = Lua::new();

    let global = lua.globals();
    global.set(
        "alert",
        lua.create_function(lua_alert).unwrap()
    ).unwrap();

    global.set(
        "write",
        lua.create_function(lua_write).unwrap()
    ).unwrap();

    global.set(
        "find_element",
        lua.create_function(lua_query_selector).unwrap()
    ).unwrap();

    Box::into_raw(Box::new(lua))
}

#[no_mangle]
pub unsafe extern "C" fn lua_execute(lua: *mut Lua, to_execute: *const c_char) {
    let lua: &mut Lua = &mut *lua;
    let to_execute = CStr::from_ptr(to_execute);
    if let Err(err) = lua.load(&to_execute.to_string_lossy().to_string()).exec() {
        eprintln!("{}", err);
    }
}