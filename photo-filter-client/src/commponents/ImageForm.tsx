import React, { useState, useRef } from 'react';

interface FileDropProps {
    onImageSet: (file: File) => void;
}

const FileDrop: React.FC<FileDropProps> = ({ onImageSet }) => {
    const [isDragActive, setIsDragActive] = useState(false);
    const [selectedImage, setSelectedImage] = useState<File | null>(null);
    const inputRef = useRef<HTMLInputElement>(null);

    const handleDragOver = (event: React.DragEvent) => {
        event.preventDefault();
        setIsDragActive(true);
    };

    const handleDragLeave = () => {
        setIsDragActive(false);
    };

    const handleDrop = (event: React.DragEvent) => {
        event.preventDefault();
        setIsDragActive(false);
        if (event.dataTransfer.files && event.dataTransfer.files.length > 0) {
            const file = event.dataTransfer.files[0];
            setSelectedImage(file);
            onImageSet(file);
        }
    };

    const handleClick = () => {
        inputRef.current?.click();
    };

    return (
        <div
            className={`w-9/10 max-h-[500px] border-2 border-dashed rounded-md p-6 text-center cursor-pointer overflow-hidden ${isDragActive ? 'border-blue-500' : 'border-gray-300'
                }`}
            onDragOver={handleDragOver}
            onDragLeave={handleDragLeave}
            onDrop={handleDrop}
            onClick={handleClick}
        >
            <input
                type="file"
                ref={inputRef}
                accept="image/*"
                style={{ display: 'none' }}
                onChange={(e) => {
                    if (e.target.files) {
                        const file = e.target.files[0];
                        setSelectedImage(file);
                        onImageSet(file);
                    }
                }}
            />
            <p>ファイル添付 またはここにファイルをドロップ</p>
            {selectedImage && (
                <img
                    src={URL.createObjectURL(selectedImage)}
                    alt="Selected"
                    className="object-contain max-h-[450px] w-full pt-6"
                />
            )}
        </div>
    );
};

export default FileDrop;