import { FC, Fragment, MouseEventHandler, ReactNode, useEffect, useRef, useState } from "react";
import { Menu, Transition } from "@headlessui/react";
import { ChevronDownIcon } from "@heroicons/react/20/solid";
import { classNames } from "@/lib/utils";

interface DropdownItem {
  text?: string;
  leftIcon?: ReactNode;
  rightIcon?: ReactNode;
  onClick: MouseEventHandler<HTMLButtonElement>;
}

export interface DropdownMenuProps {
  text: ReactNode;
  dropdownItems: DropdownItem[];
  buttonColor?: string;
}

export const DropdownMenu: FC<DropdownMenuProps> = ({
  text = "",
  dropdownItems,
  buttonColor = "white",
}) => {
  const buttonRef = useRef(null);
  const [dropdownPosition, setDropdownPosition] = useState({ top: 0, left: 0 });

  // Function to position the dropdown menu
  const positionDropdownMenu = () => {
    if (buttonRef.current) {
      const buttonRect = buttonRef.current.getBoundingClientRect();
      setDropdownPosition({ top: buttonRect.bottom, left: buttonRect.left });
    }
  };

  // Call the function initially and whenever the window is resized or scrolled
  useEffect(() => {
    positionDropdownMenu();
    console.log(dropdownPosition);
    window.addEventListener('resize', positionDropdownMenu);
    window.addEventListener('scroll', positionDropdownMenu);
    return () => {
      window.removeEventListener('resize', positionDropdownMenu);
      window.removeEventListener('scroll', positionDropdownMenu);
    };
  }, []);
  
  const buttonClass = `inline-flex w-full justify-center gap-x-1.5 rounded-md bg-transparent pl-3 px-1 py-2 text-sm font-semibold text-${buttonColor}-900`;
  return (
    <Menu as="div" className="relative inline-block text-left">
      <div>
        <Menu.Button className={buttonClass} ref={buttonRef}
        onClick={() => {
          positionDropdownMenu();
        }}
        >
          {text}
        </Menu.Button>
      </div>

      <Transition
        as={Fragment}
        enter="transition ease-out duration-100"
        enterFrom="transform opacity-0 scale-95"
        enterTo="transform opacity-100 scale-100"
        leave="transition ease-in duration-75"
        leaveFrom="transform opacity-100 scale-100"
        leaveTo="transform opacity-0 scale-95"
      >
        <Menu.Items 
        style={{ position: 'fixed', top: dropdownPosition.top, left: dropdownPosition.left }}
        className="mt-2 w-56 origin-top-right rounded-md bg-white focus:outline-none"
        // className={`fixed top-${dropdownPosition.top} left-${dropdownPosition.left} tran mt-2 w-56 origin-top-right rounded-md bg-white ring-1 ring-black ring-opacity-5 focus:outline-none`}
        >
          <div className="py-1">
            {dropdownItems.map((item, index) => (
              <Menu.Item key={index}>
                {({ active }) => (
                  <>
                    {item.leftIcon}
                    <button
                      type="button"
                      className={classNames(
                        "text-gray-700 hover:bg-gray-100",
                        "block w-full px-4 py-2 text-left text-sm"
                      )}
                      onClick={item.onClick}
                    >
                      {item.text}
                    </button>
                    {item.leftIcon}
                  </>
                )}
              </Menu.Item>
            ))}
          </div>
        </Menu.Items>
      </Transition>
    </Menu>
  );
};
