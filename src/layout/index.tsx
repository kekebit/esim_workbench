import styles from "./index.module.less";
import { Outlet } from "react-router";
export const Layout = () => {
  return (
    <div className={styles.layout}>
      <div className={styles.header}></div>
      <div className={styles.body}>
        <div className={styles.aside}></div>
        <div className={styles.content}>
          <Outlet />
        </div>
      </div>
    </div>
  );
};
