// App.js
import React, { useState } from "react";
import axios from "axios";
import "./App.css";
import mach from "./mach_plot.png";
import temp from "./temp_plot.png";
import press from "./pressure_plot.png";
import InputSection from "./component/InputSection";

function App() {
  const [inputParameters, setInputParameters] = useState({
    initial_mach: 0,
    nozzle_angle: 0,
    initial_pressure: 0,
    initial_temperature: 0,
  });

  const [outputProperties, setOutputProps] = useState(false);

  const handleInputChange = (e) => {
    const { name, value } = e.target;
    setInputParameters((prevParams) => ({
      ...prevParams,
      [name]: parseFloat(value),
    }));
  };

  const calculateProperties = async () => {
    if (inputParameters.initial_mach >= 1) {
      alert("Error: Inlet Mach Number cannot be greater than 1.");
      return;
    }

    try {
      const response = await axios.post(
        "http://localhost:8000/calculate_properties",
        inputParameters
      );
      console.log(inputParameters);
      setOutputProps(true);
    } catch (error) {
      console.error("Error calculating properties:", error);
    }
  };
  const resetProperties = () => {
    setInputParameters({
      initial_mach: 0,
      nozzle_angle: 0,
      initial_pressure: 0,
      initial_temperature: 0,
    });
    setOutputProps(false);
  };

  return (
    <div className="container">
      <h1>Compressible Flow Calculator</h1>

      {/* Either display the input section or the graph sections based on outputProperties */}
      {outputProperties ? (
        <>
          <div className="out">
            <div className="output-section">
              <h2>Mach Number Variation</h2>
              <img src={mach} alt="Mach Number" height={400} width={500} />
            </div>
            <div className="output-section">
              <h2>Temperature Variation</h2>
              <img src={temp} alt="Temperature" height={400} width={500} />
            </div>
            <div className="output-section">
              <h2>Pressure Variation</h2>
              <img src={press} alt="Pressure" height={400} width={500} />
            </div>
          </div>
          <button onClick={resetProperties}>Recalculate Properties</button>
        </>
      ) : (
        <InputSection
          inputParameters={inputParameters}
          handleInputChange={handleInputChange}
          calculateProperties={calculateProperties}
        />
      )}
    </div>
  );
}

export default App;
