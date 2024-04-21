export const prerender = true;
export const ssr = false;

export const load = async ({ url }: { url: URL }) => {
  const { pathname } = url;

  return {
    pathname,
  };
};
