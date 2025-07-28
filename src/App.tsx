
import "./App.css";
import ProcessTable from "./components/ProcessTable.tsx";

function App() {



  return (
      <div className="App flex flex-col gap-3 items-center font-mono w-screen h-screen ">
            {/* <h1 className="text-4xl font-bold w-screen h-10 text-center my-3">NetWork Address UI</h1> */}
            <div className="w-screen f-screen flex-grow px-3 py-1 flex-wrap overflow-y-scroll">
                <ProcessTable />
            </div>
      </div>
  );
}

export default App;
