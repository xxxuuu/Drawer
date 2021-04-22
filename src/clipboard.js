import db from './db';

function get() {
  return db.getAll();
}

function store(value) {
  db.store(value);
}

export default { store, get };
