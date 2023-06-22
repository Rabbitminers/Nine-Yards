<template>
<div class="app-container">
    <div class="app-header">
        <div class="app-header-left">
        <span class="app-icon"></span>
        <p class="app-name">Nine Yards</p>
        <div class="search-wrapper">
            <input class="search-input" type="text" placeholder="Search">
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" class="feather feather-search" viewBox="0 0 24 24">
            <defs></defs>
            <circle cx="11" cy="11" r="8"></circle>
            <path d="M21 21l-4.35-4.35"></path>
            </svg>
        </div>
        </div>
        <div class="app-header-right">
        <theme/>
        <button class="add-btn" title="Add New Project">
            <svg class="btn-icon feather feather-plus" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round">
            <line x1="12" y1="5" x2="12" y2="19" />
            <line x1="5" y1="12" x2="19" y2="12" /></svg>
        </button>
        <button class="notification-btn">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-bell">
            <path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9" />
            <path d="M13.73 21a2 2 0 0 1-3.46 0" /></svg>
        </button>
        <button class="profile-btn">
            <img src="https://avatars.githubusercontent.com/u/79579164?s=400&u=2d32d51075ebfd7e1ca5489b27952a7de15cdb49&v=4" />
            <span>Rabbitminers</span>
        </button>
        </div>
        <button class="updates-btn">
        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-update-circle">
            <path d="M21 11.5a8.38 8.38 0 0 1-.9 3.8 8.5 8.5 0 0 1-7.6 4.7 8.38 8.38 0 0 1-3.8-.9L3 21l1.9-5.7a8.38 8.38 0 0 1-.9-3.8 8.5 8.5 0 0 1 4.7-7.6 8.38 8.38 0 0 1 3.8-.9h.5a8.48 8.48 0 0 1 8 8v.5z" /></svg>
        </button>
    </div>
    <div class="app-content">
        <div class="app-sidebar">
        <router-link active-class="active" to="about" href="" class="app-sidebar-link">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-home">
            <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" />
            <polyline points="9 22 9 12 15 12 15 22" /></svg>
        </router-link>
        <router-link active-class="active" to="board" class="app-sidebar-link">
            <svg class="link-icon feather feather-pie-chart" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" viewBox="0 0 24 24">
            <defs />
            <path d="M21.21 15.89A10 10 0 118 2.83M22 12A10 10 0 0012 2v10z" />
            </svg>
        </router-link>
        <router-link active-class="active" to="calendar" class="app-sidebar-link">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-calendar">
            <rect x="3" y="4" width="18" height="18" rx="2" ry="2" />
            <line x1="16" y1="2" x2="16" y2="6" />
            <line x1="8" y1="2" x2="8" y2="6" />
            <line x1="3" y1="10" x2="21" y2="10" /></svg>
        </router-link>
        <router-link active-class="active" to="settings" class="app-sidebar-link">
            <svg class="link-icon feather feather-settings" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" viewBox="0 0 24 24">
            <defs />
            <circle cx="12" cy="12" r="3" />
            <path d="M19.4 15a1.65 1.65 0 00.33 1.82l.06.06a2 2 0 010 2.83 2 2 0 01-2.83 0l-.06-.06a1.65 1.65 0 00-1.82-.33 1.65 1.65 0 00-1 1.51V21a2 2 0 01-2 2 2 2 0 01-2-2v-.09A1.65 1.65 0 009 19.4a1.65 1.65 0 00-1.82.33l-.06.06a2 2 0 01-2.83 0 2 2 0 010-2.83l.06-.06a1.65 1.65 0 00.33-1.82 1.65 1.65 0 00-1.51-1H3a2 2 0 01-2-2 2 2 0 012-2h.09A1.65 1.65 0 004.6 9a1.65 1.65 0 00-.33-1.82l-.06-.06a2 2 0 010-2.83 2 2 0 012.83 0l.06.06a1.65 1.65 0 001.82.33H9a1.65 1.65 0 001-1.51V3a2 2 0 012-2 2 2 0 012 2v.09a1.65 1.65 0 001 1.51 1.65 1.65 0 001.82-.33l.06-.06a2 2 0 012.83 0 2 2 0 010 2.83l-.06.06a1.65 1.65 0 00-.33 1.82V9a1.65 1.65 0 001.51 1H21a2 2 0 012 2 2 2 0 01-2 2h-.09a1.65 1.65 0 00-1.51 1z" />
            </svg>
        </router-link>
        </div>

        <router-view/>
    </div>
    </div>
</template>

<script setup lang="ts">
import theme from '~/components/ui/theme.vue';
</script>

<style lang="scss">;

.app {
  &-container {
    width: 100%;
    display: flex;
    flex-direction: column;
    height: 100%;
    background-color: var(--app-container);
    transition: .2s;
    max-width: 1800px;
    
    button, input, optgroup, select, textarea {
      font-family: 'DM Sans', sans-serif;
    }
  }
  
  &-content {
    display: flex;
    height: 100%;
    overflow: hidden;
    padding: 16px 24px 24px 0;
  }
  
  &-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    width: 100%;
    padding: 16px 24px;
    position: relative;
    
    &-left, &-right {
      display: flex;
      align-items: center;
    }
    
    &-left { flex-grow: 1; } 
    
    &-right button {
      margin-left: 10px; 
    }
  }
  
  &-icon {
    width: 26px;
    height: 2px;
    border-radius: 4px;
    background-color: var(--main-color);
    position: relative;
    
    &:before, &:after {
      content: '';
      position: absolute;
      width: 12px;
      height: 2px;
      border-radius: 4px;
      background-color: var(--main-color);
      left: 50%;
      transform: translatex(-50%);
    }
    
    &:before { top: -6px; }
    &:after { bottom: -6px; }
  }
  
  &-name {
    color: var(--main-color);
    margin: 0;
    font-size: 20px;
    line-height: 24px;
    font-weight: 700;
    margin: 0 32px;
  }
}

.mode-switch {
  background-color: transparent;
  border: none;
  padding: 0;
  color: var(--main-color);
  display: flex;
  justify-content: center;
  align-items: center;
}

.search-wrapper {
  border-radius: 20px;
  background-color: var(--search-area-bg);
  padding-right: 12px;
  height: 40px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
  max-width: 480px;
  color: var(--light-font);
  box-shadow: 0 2px 6px 0 rgba(136,148,171,.2),0 24px 20px -24px rgba(71,82,107,.1);
  overflow: hidden;
  
  .dark & { box-shadow: none; }
}

.search-input {
  border: none;
  flex: 1;
  outline: none;
  height: 100%;
  padding: 0 20px;
  font-size: 16px;
  background-color: var(--search-area-bg);
  color: var(--main-color);
  
  &:placeholder {
    color: var(--main-color);
    opacity: .6;
  }
}

.add-btn {
  color: #fff;
  background-color: var(--button-bg);
  padding: 0;
  border: 0;
  border-radius: 50%;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.notification-btn {
  color: var(--main-color);
  padding: 0;
  border: 0;
  background-color: transparent;
  height: 32px;
  display: flex;
  justify-content: center;
  align-items: center;
}

.profile-btn {
  padding: 0;
  border: 0;
  background-color: transparent;
  display: flex;
  align-items: center;
  padding-left: 12px;
  border-left: 2px solid #ddd;
  
  img {
    width: 32px;
    height: 32px;
    object-fit: cover;
    border-radius: 50%;
    margin-right: 4px;
  }
  
  span {
    color: var(--main-color);
    font-size: 16px;
    line-height: 24px;
    font-weight: 700;
  }
}

.page-content {
  flex: 1;
  width: 100%;
}

.app-sidebar {
  padding: 40px 16px;
  display: flex;
  flex-direction: column;
  align-items: center;
  
  &-link {
    color: var(--main-color);
    color: var(--link-color);
    margin: 16px 0;
    transition: .2s;
    border-radius: 50%;
    flex-shrink: 0;
    width: 40px;
    height: 40px;
    display: flex;
    justify-content: center;
    align-items: center;
    
    &:hover {
      background-color: var(--link-color-hover);
      color: var(--link-color-active);
    }

    &.active {
      background-color: var(--link-color-active-bg);
      color: var(--link-color-active);
    }
  }
}

.projects-section {
  flex: 2;
  background-color: var(--projects-section);
  border-radius: 32px;
  padding: 32px 32px 0 32px;
  overflow: hidden;
  height: 100%;
  display: flex;
  flex-direction: column;
  
  &-line {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-bottom: 32px;
  }
  
  &-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 24px;
    color: var(--main-color);
    
    p {
      font-size: 24px;
      line-height: 32px;
      font-weight: 700;
      opacity: .9;
      margin: 0;
      color: var(--main-color);
    }
    
    .time {
      font-size: 20px;
    }
  }
}

.projects-status {
  display: flex;
}

.view-actions {
  display: flex;
  align-items: center;
}


.view-btn {
  width: 36px;
  height: 36px;
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 6px;
  border-radius: 4px;
  background-color: transparent;
  border: none;
  color: var(--main-color);
  margin-left: 8px;
  transition: .2s;
  
  &.active {
    background-color: var(--link-color-active-bg);
    color: var(--link-color-active);
  }
  
  &:not(.active):hover {
    background-color:  var(--link-color-hover);
    color: var(--link-color-active);
  }
}

.project-boxes {
  margin: 0 -8px;
  overflow-y: auto;
  
  &.jsGridView {
    display: flex;
    flex-wrap: wrap;
    
    .project-box-wrapper {
      width: 33.3%;
    }
  }
  
  &.jsListView {
    .project-box {
      display: flex;
      border-radius: 10px;
      position: relative;
      > * {
        margin-right: 24px;
      }
    }
    
    .more-wrapper {
      position: absolute;
      right: 16px;
      top: 16px;
    }
    
    .project-box-content-header {
      order: 1;
      max-width: 120px;
    }
    
    .project-box-header {
      order: 2;
    }
    
    .project-box-footer {
      order: 3;
      padding-top: 0;
      flex-direction: column;
      justify-content: flex-start;
    }
    
    .project-box-footer:after {
      display: none;
    }
    
    .participants {
      margin-bottom: 8px;
    }
    
    .project-box-content-header p {
      text-align: left;
      overflow: hidden;
      white-space: nowrap;
      text-overflow: ellipsis;
    }
    
    .project-box-header > span {
      position: absolute;
      bottom: 16px;
      left: 16px;
      font-size: 12px;
    }
    
    .box-progress-wrapper {
      order: 3;
      flex: 1;
    }
  }
}

.project-box {
  --main-color-card: #dbf6fd;
  border-radius: 30px;
  padding: 16px;
  background-color: var(--main-color-card);
  
  &-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 16px;
    color: var(--main-color);
    
    span {
      color: #4A4A4A;
      opacity: .7;
      font-size: 14px;
      line-height: 16px;
    }
  }
  
  &-content-header {
    text-align: center;
    margin-bottom: 16px;
    
    p { margin: 0; }
  }
  
  &-wrapper {
    padding: 8px;
    transition: .2s;
  }
}

.project-btn-more {
  padding: 0;
  height: 14px;
  width: 24px;
  height: 24px;
  position: relative;
  background-color: transparent;
  border: none;
  flex-shrink: 0;
}

.more-wrapper {
  position: relative;
}

.box-content-header {
  font-size: 16px;
  line-height: 24px;
  font-weight: 700;
  opacity: .7;
}

.box-content-subheader {
  font-size: 14px;
  line-height: 24px;
  opacity: .7;
}

.box-progress {
  display: block;
  height: 4px;
  border-radius: 6px;
  
  &-bar {
    width: 100%;
    height: 4px;
    border-radius: 6px;
    overflow: hidden;
    background-color: #fff;
    margin: 8px 0;
  }
  
  &-header {
    font-size: 14px;
    font-weight: 700;
    line-height: 16px;
    margin: 0;
  }
  
  &-wrapper {
    
  }
  
  &-percentage {
    text-align: right;
    margin: 0;
    font-size: 14px;
    font-weight: 700;
    line-height: 16px;
  }
}

.project-box-footer {
  display: flex;
  justify-content: space-between;
  padding-top: 16px;
  position: relative;
  
  &:after {
    content: '';
    position: absolute;
    background-color: rgba(255,255,255,0.6);;
    width: calc(100% + 32px);
    top: 0;
    left: -16px;
    height: 1px;
  }
}

.participants {
  display: flex;
  align-items: center;
  
  img {
    width: 20px;
    height: 20px;
    border-radius: 50%;
    overflow: hidden;
    object-fit: cover;
    
    &:not(:first-child) {
      margin-left: -8px;
    }
  }
}

.add-participant {
  width: 20px;
  height: 20px;
  border-radius: 50%;
  border: none;
  background-color: rgba(255,255,255,0.6);
  margin-left: 6px;
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 0;
}

.days-left {
  background-color: rgba(255,255,255,0.6);
  font-size: 12px;
  border-radius: 20px;
  flex-shrink: 0;
  padding: 6px 16px;
  font-weight: 700;
}

.moon.active {
  fill: var(--main-color);
}

@media screen and (max-width: 980px) {
  .project-boxes.jsGridView .project-box-wrapper {
    width: 50%;
  }
}

@media screen and (max-width: 720px) {
  .app-name, .profile-btn span { display: none; }
  
  .add-btn, .notification-btn, .mode-switch {
    width: 20px;
    height: 20px;
    
    svg {
      width: 16px;
      height: 16px;
    }
  }
  
  .app-header-right button {
    margin-left: 4px;
  }
}

@media screen and (max-width: 520px) {
  .projects-section { overflow: auto; }
  .project-boxes { overflow-y: visible; }
  
  .app-sidebar, .app-icon { display: none; }
  
  .app-content { padding: 16px 12px 24px 12px;}
  
  .view-btn {
    width: 24px;
    height: 24px;
  }
  
  .app-header {
    padding: 16px 10px;
  }
  
  .search-input {
    max-width: 120px;
  }
  
  .project-boxes.jsGridView .project-box-wrapper {
    width: 100%;
  }
  
  .projects-section {
    padding: 24px 16px 0 16px;
  }
  
  .profile-btn img {
    width: 24px;
    height: 24px;
  }
  
  .app-header {
    padding: 10px;
  }
  
  .projects-section-header p,
  .projects-section-header .time{
    font-size: 18px;
  }
  
  .search-input {
    font-size: 14px;
  }

  .box-content-header {
    font-size: 12px;
    line-height: 16px;
  }

  .box-content-subheader {
    font-size: 12px;
    line-height: 16px;
  }

  .project-boxes.jsListView .project-box-header > span {
    font-size: 10px;
  }

  .box-progress-header {
    font-size: 12px;
  }

  .box-progress-percentage {
    font-size: 10px;
  }

  .days-left {
    font-size: 8px;
    padding: 6px 6px;
    text-align: center;
  }

  .project-boxes.jsListView .project-box > * {
    margin-right: 10px;
  }

  .project-boxes.jsListView .more-wrapper {
    right: 2px;
    top: 10px;
  }
}
</style>