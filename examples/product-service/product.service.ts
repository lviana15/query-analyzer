import { Injectable } from '@nestjs/common';
import { Model } from 'mongoose';
import { InjectModel } from '@nestjs/mongoose';
import { Product } from './schemas/product.schema';

@Injectable()
export class ProductService {
  constructor(@InjectModel(Product.name) private productModel: Model<Product>) {}

  async findProductsByCategory(category: string) {
    return this.productModel.find({
      category: category,
      inStock: true
    }).exec();
  }

  async findProductById(id: string) {
    return this.productModel.findOne({
      _id: id,
      isActive: true
    }).exec();
  }

  async searchProducts(searchTerm: string) {
    return this.productModel.find({
      $and: [
        { isActive: true },
        { $or: [
          { name: { $regex: searchTerm, $options: 'i' } },
          { description: { $regex: searchTerm, $options: 'i' } }
        ]}
      ]
    }).exec();
  }

  async findProductsByPriceRange(min: number, max: number) {
    return this.productModel.find({
      price: { $gte: min, $lte: max },
      inStock: true
    }).exec();
  }

  async getPopularProducts(limit: number = 5) {
    return this.productModel.find({
      rating: { $gte: 4.0 },
      inStock: true
    }).sort({ rating: -1 }).limit(limit).exec();
  }

  async countProductsByCategory(category: string) {
    return this.productModel.countDocuments({
      category: category,
      isActive: true
    }).exec();
  }

  async getProductStats() {
    return this.productModel.aggregate([
      { $match: { isActive: true } },
      { $group: { 
        _id: '$category', 
        avgPrice: { $avg: '$price' },
        count: { $sum: 1 }
      }}
    ]).exec();
  }
}