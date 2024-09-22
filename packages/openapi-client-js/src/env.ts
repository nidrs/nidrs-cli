export default {
  get DEBUG() {
    return (window as any).DEBUG ?? false;
  },
};
