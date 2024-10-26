import React, { useState } from 'react';
import ButtonGroup from './commponents/ButtonGroup';
import ImageForm from './commponents/ImageForm';
import GrayScaleButton from './commponents/GrayScaleButton';
import OddPixelButton from './commponents/OddPixelButton';
import Film1Button from './commponents/Film1Button';

const App: React.FC = () => {
  const film_fillter = 'フィルムシミュレーター';
  const grayscale_fillter = 'gray scale';
  const odd_pixel_fillter = 'odd pixel';
  const fillters = [film_fillter, grayscale_fillter, odd_pixel_fillter];

  const [selectedButton, setSelectedButton] = useState(fillters[0]);
  const [selectedImage, setSelectedImage] = useState<File | null>(null);
  const [filterdImage, seFilterdImage] = useState<File | null>(null);

  const handleSelect = (button: string) => {
    setSelectedButton(button);
  };

  return (
    <div className="p-8">
      <h1 className='flex flex-col items-center text-2xl mb-6'>
        wasm Photo Filter
      </h1>
      {/* 画像セレクト */}
      <div className='w-9/10 max-h-[500px]'>
        <ImageForm onImageSet={setSelectedImage} />
      </div>
      {/* フィルターセレクト */}
      <div className='flex flex-col items-center p-5'>
        <ButtonGroup
          buttons={fillters}
          selected={selectedButton}
          onSelect={handleSelect}
        />
      </div>
      <div>
        {/* フィルター毎の設定&実行ボタン */}
        {selectedButton === grayscale_fillter
          && <GrayScaleButton
            imageFile={selectedImage}
            setProcessedImage={seFilterdImage} />}

        {selectedButton === film_fillter
          && <Film1Button
            imageFile={selectedImage}
            setProcessedImage={seFilterdImage} />}

        {selectedButton === odd_pixel_fillter
          && <OddPixelButton
            imageFile={selectedImage}
            setProcessedImage={seFilterdImage} />}

      </div>
      <div className='p-6 flex flex-col items-center'>
        {/* フィルター後の画像表示 */}
        {filterdImage && (
          <img
            src={URL.createObjectURL(filterdImage)}
            alt="filterd"
            className="max-h-[450px]"
          />
        )}
      </div>
    </div >
  );
};

export default App;
