<div class="inline-flex rounded-md shadow-sm" role="group">
  <button
    type="button"
    class="px-4 py-2 text-sm font-medium text-gray-900 bg-white border border-gray-200 rounded-s-lg hover:bg-gray-100 hover:text-blue-700 focus:z-10 focus:ring-2 focus:ring-blue-700 focus:text-blue-700 dark:bg-gray-800 dark:border-gray-700 dark:text-white dark:hover:text-white dark:hover:bg-gray-700 dark:focus:ring-blue-500 dark:focus:text-white"
  >
    View
  </button>
  <button
    id="dropdownMenuIconButton"
    data-dropdown-toggle="dropdownDots"
    class="inline-flex items-center p-2 text-sm font-medium text-center text-gray-900 bg-white rounded-e-lg hover:bg-gray-100 focus:ring-4 focus:outline-none dark:text-white focus:ring-gray-50 dark:bg-gray-800 dark:hover:bg-gray-700 dark:focus:ring-gray-600"
    type="button"
  >
    <svg
      class="w-5 h-5"
      aria-hidden="true"
      xmlns="http://www.w3.org/2000/svg"
      fill="currentColor"
      viewBox="0 0 4 15"
    >
      <path
        d="M3.5 1.5a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0Zm0 6.041a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0Zm0 5.959a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0Z"
      />
    </svg>
  </button>
</div>

<div
  id="dropdownDots"
  class="z-10 hidden bg-white divide-y divide-gray-100 rounded-lg shadow w-44 dark:bg-gray-700 dark:divide-gray-600"
>
  <ul
    class="py-2 text-sm text-gray-700 dark:text-gray-200"
    aria-labelledby="dropdownMenuIconButton"
  >
    <li>
      <a
        href="#"
        class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white"
        >Modify</a
      >
    </li>
    <li>
      <a
        href="#"
        class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white"
        >Delete</a
      >
    </li>
  </ul>
</div>
