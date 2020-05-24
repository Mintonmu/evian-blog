//
//  ArticleViewModel.swift
//  EvianBlog
//
//  Created by Evian张 on 2020/5/23.
//  Copyright © 2020 Evian张. All rights reserved.
//

import Foundation

class ArticleViewModel: ObservableObject, WritingsSubviewDelegate {
	@Published var level: WritingsSubviewLevel = .total
	@Published var currentDetailViewIndex = 0
	
	private let blogAPI: BlogAPI
	var totalViewModel: ArticleTotalViewModel
	var detailViewModel: DetailPageViewModel<ArticleDetailView>
	
	init(blogAPI: BlogAPI) {
		self.blogAPI = blogAPI
		self.totalViewModel = ArticleTotalViewModel(blogAPI: self.blogAPI)
		self.detailViewModel = DetailPageViewModel([])
	}
	
	func currentLevel() -> WritingsSubviewLevel {
		self.level
	}
	
	func changeLevel(to writingsSubviewLevel: WritingsSubviewLevel) {
		self.level = writingsSubviewLevel
	}
	
	func navigateToDetailPage(name: String) {
		if let targetPage = self.detailViewModel.hasView(where: { articleDetailView in
			return articleDetailView.viewModel.articleTitle == name
		}) {
			self.detailViewModel.currentPage = targetPage
		} else {
			self.detailViewModel.addView(ArticleDetailView(articleDetailViewModel: ArticleDetailViewModel(blogAPI: self.blogAPI, articleTitle: name)))
		}
	}
	
	func isCurrentViewClosable() -> Bool {
		self.level == .detail && !self.detailViewModel.viewControllers.isEmpty
	}
	
	func closeCurrentView() {
		if isCurrentViewClosable() {
			self.detailViewModel.deleteCurrentView()
		}
	}
}
