# Steps to reproduce loco.rs issue #1555

### These steps are not necesarily all required in order to reproduce, but they aim to be the minimally necessary steps to reproduce

## 1. Set up a new saas app with server side rendering and sqllite
(base) simon@POSEIDON:~/git$ loco new

✔ ❯ App name? · recursive_furnace_reproduce_minimal

✔ ❯ What would you like to build? · Saas App with server side rendering

✔ ❯ Select a DB Provider · Sqlite

✔ ❯ Select your background worker type · Async (in-process tokio async tasks)

🚂 Loco app generated successfully in:
/home/simon/git/recursive_furnace_reproduce_minimal

## 2. Run initial project once