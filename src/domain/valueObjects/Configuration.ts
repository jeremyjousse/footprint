export type Configuration = {
  application: ApplicationConfiguration;
};

export type ApplicationConfiguration = {
  consultationLocations: string[];
  consultationStatuses: string[];
};

export const defaultApplicationConfiguration = (): ApplicationConfiguration => {
  return {
    consultationLocations: [],
    consultationStatuses: [],
  };
};
