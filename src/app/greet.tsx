'use client'

import { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri'
import Button from '@/components/buttons/Button';

export default function Greet() {
  const [greeting, setGreeting] = useState('Next.js');
  const [textValue, setTextValue] = useState('');

  // Necessary because we will have to use Greet as a component later.
  return (
    <div>
      <div>
        <form>
          <label>Name: </label>
          <input
            type='text'
            value={textValue}
            className='text-black'
            onChange={
              (e) => {
                setTextValue(e.target.value);
                console.log(e.target.value);
                console.log(textValue);
              }
            }
          />
        </form>
        <Button
          onClick={(e) => {
            e.preventDefault();
            // invoke<string>('greet2', { name: textValue })
            //   .then(result => {
            //     setGreeting(result);
            //     console.log(result); // Log the updated value of greeting
            //   })
            //   .catch(console.error)
          }
          }
        >
          <p>button</p>
        </Button>
      </div>
      <div>
        {greeting ?? ""}
      </div>
    </div>);
}