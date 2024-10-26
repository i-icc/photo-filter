// Slider.tsx
import React, { useState } from 'react';

interface SliderProps {
    min: number;
    max: number;
    value: number;
    onChange: (value: number) => void;
}

const Slider: React.FC<SliderProps> = ({ min, max, value, onChange }) => {
    const [currentValue, setCurrentValue] = useState(value);

    const handleInputChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        const newValue = parseInt(event.target.value);
        if (newValue >= min && newValue <= max) {
            setCurrentValue(newValue);
            onChange(newValue);
        }
    };

    return (
        <div className="flex items-center w-9/10">
            <span className="mr-2"></span>
            <input
                type="range"
                min={min}
                max={max}
                value={currentValue}
                onChange={handleInputChange}
                className="w-full"
            />
            <span className="ml-2"></span>
        </div>
    );
};

export default Slider;