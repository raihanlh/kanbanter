import { FC, Fragment, MouseEventHandler, ReactNode } from "react";
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
  const buttonClass = `inline-flex w-full justify-center gap-x-1.5 rounded-md bg-transparent pl-3 px-1 py-2 text-sm font-semibold text-${buttonColor}-900 shadow-sm`;
  return (
    <Menu as="div" className="relative inline-block text-left">
      <div>
        <Menu.Button className={buttonClass}>
          {text}
          {/* <ChevronDownIcon
            className="-mr-1 h-5 w-5 text-gray-400"
            aria-hidden="true"
          /> */}
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
        <Menu.Items className="absolute right-0 z-10 mt-2 w-56 origin-top-right rounded-md bg-white shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none">
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
