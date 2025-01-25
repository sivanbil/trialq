export default {
    install(app) {
        // 获取当前年份
        const currentYear = new Date().getFullYear();
        app.config.globalProperties.$slogan = {
            main: 'TrialQ',
            sub: '临研数据分析『超跑引擎』全速驱动突破，效率一路狂飙',
            year: currentYear, // 自动获取的当前年份
            email: 'trialqtool@gmail.com', // 固定的电子邮件地址
        };
    },
};