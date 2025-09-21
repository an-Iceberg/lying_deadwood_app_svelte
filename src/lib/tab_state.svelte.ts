enum Tab {
  One_time,
  Input,
  Data,
}

let current_tab = $state(Tab.One_time);

export function is_one_time(): boolean {
  return current_tab === Tab.One_time;
}

export function is_input(): boolean {
  return current_tab === Tab.Input;
}

export function is_data(): boolean {
  return current_tab === Tab.Data;
}

export function to_one_time() {
  current_tab = Tab.One_time;
}

export function to_input() {
  current_tab = Tab.Input;
}

export function to_data() {
  current_tab = Tab.Data;
}

// $inspect(current_tab)
