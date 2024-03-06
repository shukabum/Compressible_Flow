// InputSection.js
import React from "react";

const InputSection = ({
  inputParameters,
  handleInputChange,
  calculateProperties,
}) => {
  return (
    <div className="input-section">
      <label>
        Inlet Mach Number
        <input
          type="number"
          name="initial_mach"
          value={inputParameters.initial_mach}
          onChange={handleInputChange}
          placeholder="Enter Inlet Mach Number"
        />
      </label>

      <label>
        Angle (degrees)
        <input
          type="number"
          name="nozzle_angle"
          value={inputParameters.nozzle_angle}
          onChange={handleInputChange}
          placeholder="Enter Nozzle Angle"
        />
      </label>

      <label>
        Stagnation Pressure (kPa):
        <input
          type="number"
          name="initial_pressure"
          value={inputParameters.initial_pressure}
          onChange={handleInputChange}
          placeholder="Enter Stagnation Pressure"
        />
      </label>

      <label>
        Stagnation Temperature (K):
        <input
          type="number"
          name="initial_temperature"
          value={inputParameters.initial_temperature}
          onChange={handleInputChange}
          placeholder="Enter Stagnation Temperature"
        />
      </label>

      <button onClick={calculateProperties}>Calculate Properties</button>
    </div>
  );
};

export default InputSection;
