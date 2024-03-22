export const displayRowValue = (row: any, field: string): string => {
  const fields = field.split(".");
  if (fields.length > 1) {
    return row[fields[0]][fields[1]]; // TODO handle the right number of filed
  }
  return row[field] || "";
};
