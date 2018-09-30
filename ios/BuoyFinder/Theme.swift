//
//  Theme.swift
//  HopeWaves
//
//  Created by Matthew Iannucci on 6/3/18.
//  Copyright © 2018 Matthew Iannucci. All rights reserved.
//
import UIKit

public class ThemeManager {
    public static let primaryColor = UIColor(red: 0.16, green: 0.21, blue: 0.58, alpha: 1.0);
    public static let primaryLightColor = UIColor(red: 0.37, green: 0.37, blue: 0.77, alpha: 1.0);
    public static let primaryDarkColor = UIColor(red: 0.00, green: 0.06, blue: 0.39, alpha: 1.0);
    public static let primaryTextColor = UIColor(red: 1.00, green: 1.00, blue: 1.00, alpha: 1.0);
    public static let backgroundColor = UIColor(white: 238.0, alpha: 1.0)
    
    public static func applyDefaultTheme() {
        UIApplication.shared.statusBarStyle = .default
        UINavigationBar.appearance().barTintColor = UIColor.white
        UINavigationBar.appearance().tintColor = ThemeManager.primaryColor
        UINavigationBar.appearance().titleTextAttributes = [NSAttributedStringKey.foregroundColor:ThemeManager.primaryColor]
        UINavigationBar.appearance().largeTitleTextAttributes = [NSAttributedStringKey.foregroundColor:ThemeManager.primaryColor]
        UITabBar.appearance().tintColor = ThemeManager.primaryColor
        UIPageControl.appearance().currentPageIndicatorTintColor = ThemeManager.primaryColor
        UICollectionView.appearance().backgroundColor = UIColor(red: 238.0/255.0, green: 238.0/255.0, blue: 238.0/255.0, alpha: 1.0)
        UICollectionViewCell.appearance().backgroundColor = UIColor.white
        UICollectionReusableView.appearance().backgroundColor = UIColor.white
    }
}
