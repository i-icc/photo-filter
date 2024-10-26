import React from 'react';

type ButtonGroupProps = {
    buttons: string[];
    selected: string;
    onSelect: (button: string) => void;
};

const ButtonGroup: React.FC<ButtonGroupProps> = ({ buttons, selected, onSelect }) => {
    return (
        <div className="inline-flex rounded-full border border-blue-500">
            {buttons.map((button) => (
                <button
                    key={button}
                    className={`px-4 py-2 font-semibold text-sm rounded-full focus:outline-none 
            ${selected === button ? 'bg-blue-500 text-white' : 'text-blue-500 hover:bg-blue-100'}`}
                    onClick={() => onSelect(button)}
                >
                    {button}
                </button>
            ))}
        </div>
    );
};

export default ButtonGroup;
